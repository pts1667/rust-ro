use async_trait::async_trait;
use database::{read, tx_write};

use crate::repository::{Error, HotKeyRepository, SledRepository};
use crate::server::model::hotkey::Hotkey;

#[async_trait]
impl HotKeyRepository for SledRepository {
    async fn save_hotkeys(&self, char_id: u32, hotkeys: &Vec<Hotkey>) -> Result<(), Error> {
        self.database
            .hotkeys
            .transaction(|tree| tx_write(tree, &char_id.to_be_bytes(), hotkeys))?;
        Ok(())
    }

    async fn load_hotkeys(&self, char_id: u32) -> Result<Vec<Hotkey>, Error> {
        Ok(read(&self.database.hotkeys, &char_id.to_be_bytes())?.unwrap_or_default())
    }
}
