use super::*;

use common::{
    database::raw::DbEvent,
    gacha::{GachaData, GachaR},
    setting::SettingGacha,
};
use log::debug;
use rand::prelude::*;
use tokio::spawn;

trait GachaPull {
    fn pull(&self, rng: &mut ThreadRng) -> GachaData;
    fn guaranteed(&self, rng: &mut ThreadRng) -> GachaData;
    fn multi(&self, event: &mut DbEvent, total: usize) -> MyResult<Vec<GachaData>>;
}

impl GachaPull for SettingGacha {
    fn pull(&self, rng: &mut ThreadRng) -> GachaData {
        let roll: f32 = rng.random();

        let (pool, result) = match roll {
            r if r <= 0.001 => (&self.rarity.ur, GachaR::UR), // 0.1%
            r if r <= 0.01 => (&self.rarity.ssr1, GachaR::SSR), // 0.9%
            r if r <= 0.03 => (&self.rarity.ssr2, GachaR::SSR), // 2%
            r if r <= 0.08 => (&self.rarity.sr1, GachaR::SR), // 5%
            r if r <= 0.18 => (&self.rarity.sr2, GachaR::SR), // 10%
            r if r <= 0.33 => (&self.rarity.sr3, GachaR::SR), // 15%
            r if r <= 0.63 => (&self.rarity.r1, GachaR::R),   // 30%
            _ => (&self.rarity.r2, GachaR::R),                // 37%
        };

        let code = pool.choose(rng).unwrap().clone();
        GachaData { code, result }
    }

    fn guaranteed(&self, rng: &mut ThreadRng) -> GachaData {
        let roll: f32 = rng.random();

        let (pool, result) = match roll {
            r if r <= 0.1 => (&self.rarity.ur, GachaR::UR), // 10%
            r if r <= 0.4 => (&self.rarity.ssr1, GachaR::SSR), // 30%
            _ => (&self.rarity.ssr2, GachaR::SSR),          // 60% (fixed from GachaR::R)
        };

        let code = pool.choose(rng).unwrap().clone();
        GachaData { code, result }
    }

    fn multi(&self, event: &mut DbEvent, total: usize) -> MyResult<Vec<GachaData>> {
        let cost = self.cost * total as u32;
        if cost as i32 > event.gacha {
            return Err(MyError::Custom(format!(
                "Insufficient ticket, you need to have at least {} ticket",
                self.cost
            )));
        }

        let mut current_pity = event.pity;
        let mut rng = rand::rng();
        let mut results = Vec::with_capacity(total);

        for _ in 0..total {
            current_pity += 1;

            let pull_result = if current_pity >= self.pity as i32 {
                debug!("Reached pity");
                current_pity = 0;
                self.guaranteed(&mut rng)
            } else {
                debug!("Isn't pity");
                let res = self.pull(&mut rng);
                if matches!(res.result, GachaR::SSR | GachaR::UR) {
                    debug!("Got rare pull, reset pity");
                    current_pity = 0;
                }
                res
            };

            results.push(pull_result);
        }

        event.gacha -= cost as i32;
        event.pity = current_pity;
        Ok(results)
    }
}

impl App {
    /// state pull result byte
    pub async fn command_gacha(
        &self,
        did: impl ToString,
        avatar_url: impl ToString,
        pull: usize,
    ) -> MyResult<Vec<u8>> {
        let did = did.to_string();
        let user = self.only_register_user(&did).await?;
        let mut event = self.db.fetch_event(&did).await?;
        let setting = self.setting.read().await;

        let data = setting.gacha.multi(&mut event, pull)?;

        // send database process to another thread
        let ids = vec![user.cid];
        let code = data.iter().map(|e| e.code.to_owned()).collect::<Vec<_>>();
        let db = self.db.clone();
        let proc = spawn(async move {
            db.send_distribution(&ids, &code, "Gacha Reward", "~C05 Gacha Reward")
                .await
        });

        let mut cache = self.gacha.processed_cache.write().await;
        let byte = self
            .gacha
            .raw_cache
            .pull(data, avatar_url, &self.pedia, &mut cache)
            .await?;

        // retrieve error form database thread if they has
        proc.await??;
        Ok(byte)
    }
}
