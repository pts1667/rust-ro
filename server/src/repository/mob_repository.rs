use async_trait::async_trait;

use crate::repository::model::mob_model::MobModel;
use crate::repository::{Error, MobRepository, SledRepository};

#[async_trait]
impl MobRepository for SledRepository {
    async fn get_all_mobs(&self) -> Result<Vec<MobModel>, Error> {
        self.database
            .mobs
            .iter()
            .map(|entry| {
                let (_, value) = entry?;
                Ok(serde_json::from_slice(&value)?)
            })
            .collect()
    }
}
