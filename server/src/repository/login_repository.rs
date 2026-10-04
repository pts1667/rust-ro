use async_trait::async_trait;
use database::model::AccountRecord;
use database::tx_required;
use sled::transaction::Transactional;

use crate::repository::{Error, LoginRepository, SledRepository};

#[async_trait]
impl LoginRepository for SledRepository {
    async fn login(&self, username: String, password: String) -> Result<u32, Error> {
        Ok(
            (&self.database.accounts, &self.database.account_names).transaction(|(accounts, names)| {
                let id: u32 = tx_required(names, username.as_bytes())?;
                let account: AccountRecord = tx_required(accounts, &id.to_be_bytes())?;
                if account.password != password {
                    return Err(sled::transaction::ConflictableTransactionError::Abort(Error::NotFound));
                }
                Ok(id)
            })?,
        )
    }
}
