use crate::{
    config::Config,
    subs::Subs,
    tg_bot::{TgBot, TgMessage},
};
use anyhow::{Context, Result};
use std::sync::Arc;

pub struct Notifier {
    subs: Arc<Subs>,
    tg_bot: TgBot,
}

impl Notifier {
    pub fn init(config: Arc<Config>) -> Result<Self> {
        let subs = Arc::new(Subs::load().context("failed to load subscriptions")?);
        let tg_bot = TgBot::new(config, subs.clone());
        Ok(Notifier { subs, tg_bot })
    }

    pub async fn start_tg_command_server(&self) {
        self.tg_bot.start_command_server().await;
    }

    pub async fn broadcast(&self, message: TgMessage) -> Result<()> {
        let subs = self
            .subs
            .get_tg_subs()
            .context("failed to get telegram subscriptions")?;

        log::trace!(
            "broadcast: subs_count={subs_count}",
            subs_count = subs.len()
        );

        for sub in &subs {
            let res = self.tg_bot.send_message(sub.chat_id, &message).await;

            if let Err(error) = res {
                log::error!(
                    "failed to send message to telegram subscription {sub:?}: {error:?}",
                    sub = sub,
                    error = error
                );
            }
        }

        Ok(())
    }
}
