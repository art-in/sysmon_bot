use crate::{
    config::Config,
    notifier::Notifier,
    tg_bot::{TgMessage, TgMessageFormat},
};
use anyhow::Result;
use std::{sync::Arc, time::Duration};
use systemstat::Platform;

pub async fn start(cfg: Arc<Config>, notifier: Arc<Notifier>) -> Result<()> {
    let mut alarm_notification_broadcasted = false;

    loop {
        log::trace!("next iteration");
        let mut alarm_messages = Vec::new();

        // check cpu
        let sys = systemstat::System::new();
        let cpu_avg_load = sys.load_average()?;
        log::trace!("cpu avg load: {:?}", cpu_avg_load);
        if cpu_avg_load.fifteen > cfg.max_cpu_avg_load_15min {
            alarm_messages.push(format!(
                "average cpu load in the last 15 minutes is {load}% (max = {max}%)",
                load = cpu_avg_load.fifteen * 100.0,
                max = num_cpus::get() * 100
            ));
        }

        // check sensors
        // not using systemstat::System::cpu_temp() because it fails for some reason
        for sensor in sensors::Sensors::new() {
            let sensor_name = sensor.get_name()?;
            match cfg.sensor_thresholds_map.get(&sensor_name) {
                Some(sensor_threshold) => {
                    for feature in sensor {
                        let feature_name = feature.get_label()?;
                        match sensor_threshold.features_map.get(&feature_name) {
                            Some(feature_threshold) => {
                                for sub_feature in feature {
                                    let sub_feature_name = sub_feature.name();
                                    let sub_feature_value = sub_feature.get_value()?;

                                    if sub_feature_name == feature_threshold.sub_feature {
                                        log::trace!(
                                            "sensor: {sensor}, feature: {feature}, sub_feature = {sub_feature}, value = {value}",
                                            sensor = sensor_name,
                                            feature = feature_name,
                                            sub_feature = sub_feature_name,
                                            value = sub_feature_value
                                        );

                                        if sub_feature_value > feature_threshold.max {
                                            alarm_messages.push(format!(
                                                "sensor {feature} = {value} (max = {max})",
                                                feature = feature_name,
                                                value = sub_feature_value,
                                                max = feature_threshold.max
                                            ));
                                        }
                                    }
                                }
                            }
                            None => {
                                // ignore unknown feature
                            }
                        }
                    }
                }
                None => {
                    // ignore unknown sensor
                }
            }
        }

        if !alarm_messages.is_empty() {
            if !alarm_notification_broadcasted {
                notifier
                    .broadcast(TgMessage {
                        format: TgMessageFormat::Html,
                        text: alarm_messages.join("\n"),
                    })
                    .await?;
                alarm_notification_broadcasted = true;
            }
        } else {
            log::trace!("all readings looks good");
            alarm_notification_broadcasted = false;
        }

        tokio::time::sleep(Duration::from_secs(cfg.monitoring_interval_sec)).await;
    }
}
