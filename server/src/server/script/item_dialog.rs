use async_trait::async_trait;
use script_runtime::Host;
use script_sdk::{Function, Reply, Request};

use super::item_script_handler::{ItemScriptHost, status_variable};
use super::NpcScriptHost;

pub struct ItemDialogHost {
    pub item: ItemScriptHost,
    pub dialog: NpcScriptHost,
}

#[async_trait]
impl Host for ItemDialogHost {
    async fn invoke(&mut self, request: Request) -> Reply {
        if !self.dialog.current() { return Err("Item conversation cancelled".into()); }
        match request {
            Request::Read(ref name) if !self.item.variables.contains_key(name) && status_variable(&self.item.status, name).is_none() => {
                self.dialog.forward(request).await
            }
            Request::VariableRead { .. } => self.dialog.forward(request).await,
            Request::Call { function, .. } if matches!(function,
                Function::Mes | Function::Close | Function::Next | Function::Select | Function::InputNumber | Function::InputString
                | Function::Message | Function::DispBottom | Function::Cutin) => self.dialog.invoke(request).await,
            Request::Call { function, .. } if matches!(function, Function::GetFame | Function::GetFameRank) => self.dialog.forward(request).await,
            request => self.item.invoke(request).await,
        }
    }
}
