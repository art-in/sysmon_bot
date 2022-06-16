use anyhow::Result;
use log::LevelFilter;
use std::{collections::HashMap, fs::File, io::Read};

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    pub log_level: LevelFilter,
    pub tg_bot_token: String,
    pub monitoring_interval_sec: u64,
    pub max_cpu_avg_load_15min: f32,
    sensor_thresholds: Vec<SensorThreshold>,
    #[serde(skip)]
    pub sensor_thresholds_map: HashMap<String, SensorThreshold>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SensorThreshold {
    pub sensor: String,
    features: Vec<FeatureThreshold>,
    #[serde(skip)]
    pub features_map: HashMap<String, FeatureThreshold>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FeatureThreshold {
    pub feature: String,
    pub sub_feature: String,
    pub max: f64,
}

impl Config {
    pub fn read() -> Result<Self> {
        let mut str = String::new();
        File::open("./config.toml")?.read_to_string(&mut str)?;
        let mut config: Config = toml::from_str(&str)?;

        // build maps from vectors for faster matching while monitoring
        config.sensor_thresholds.iter().cloned().for_each(|mut s| {
            s.features.iter().cloned().for_each(|f| {
                s.features_map.insert(f.feature.clone(), f);
            });
            config.sensor_thresholds_map.insert(s.sensor.clone(), s);
        });

        Ok(config)
    }
}
