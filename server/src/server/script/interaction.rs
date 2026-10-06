use packets::packets::{
    Packet, PacketZcCloseDialog, PacketZcMenuList, PacketZcOpenEditdlg, PacketZcOpenEditdlgstr,
    PacketZcSayDialog, PacketZcShowImage2, PacketZcWaitDialog,
};
use script_sdk::{Function, Reply, Value};

use super::{NpcScriptHost, PlayerInput};
use crate::server::model::events::client_notification::{CharNotification, Notification};
use crate::util::packet::playerchat_packet;

impl NpcScriptHost {
    pub fn send_packet<'a>(&'a self, packet: &mut dyn Packet) -> impl std::future::Future<Output = Result<(), String>> + Send + 'a {
        let mut notification = Notification::Char(CharNotification::new(self.session.char_id(), std::mem::take(packet.raw_mut())));
        async move { loop {
            if !self.current() { return Err("Conversation cancelled".into()); }
            match self.notifications.try_send(notification) {
                Ok(()) => return Ok(()),
                Err(std::sync::mpsc::TrySendError::Disconnected(_)) => return Err("Notification queue is unavailable".into()),
                Err(std::sync::mpsc::TrySendError::Full(pending)) => notification = pending,
            }
            tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        } }
    }

    pub async fn interaction(&mut self, function: Function, arguments: Vec<Value>) -> Reply {
        let packetver = self.server.packetver();
        let npc = self.script.id;
        match function {
            Function::Mes => {
                let text = arguments.iter().map(Value::text).collect::<Vec<_>>().join("\n");
                if text.len() > 16_000 {
                    return Err("Dialogue is too long".into());
                }
                let mut packet = PacketZcSayDialog::new(packetver);
                packet.msg = text;
                packet.naid = npc;
                packet.packet_length = (PacketZcSayDialog::base_len(packetver) + packet.msg.len() + 1) as i16;
                packet.fill_raw();
                self.send_packet(&mut packet).await?;
            }
            Function::Close => {
                let mut packet = PacketZcCloseDialog::new(packetver);
                packet.naid = npc;
                packet.fill_raw();
                self.send_packet(&mut packet).await?;
            }
            Function::Next => {
                let mut packet = PacketZcWaitDialog::new(packetver);
                packet.naid = npc;
                packet.fill_raw();
                self.send_packet(&mut packet).await?;
                if !matches!(self.receive().await?, PlayerInput::Next) {
                    return Err("Unexpected dialogue response".into());
                }
            }
            Function::Select => {
                if arguments.is_empty() || arguments.len() > 254 {
                    return Err("Invalid number of menu options".into());
                }
                let text = arguments.iter().map(Value::text).collect::<Vec<_>>().join(":");
                if text.len() > 16_000 {
                    return Err("Menu is too long".into());
                }
                let count = text.split(':').count();
                let mut packet = PacketZcMenuList::new(packetver);
                packet.naid = npc;
                packet.msg = text;
                packet.packet_length = (PacketZcMenuList::base_len(packetver) + packet.msg.len() + 1) as i16;
                packet.fill_raw();
                self.send_packet(&mut packet).await?;
                match self.receive().await? {
                    PlayerInput::Selection(option) if option > 0 && (option as usize) <= count => return Ok(Value::Number(option as i32)),
                    other => return Err(format!("Menu cancelled or invalid selection: {other:?} with {count} options")),
                }
            }
            Function::InputNumber => {
                let mut packet = PacketZcOpenEditdlg::new(packetver);
                packet.naid = npc;
                packet.fill_raw();
                self.send_packet(&mut packet).await?;
                match self.receive().await? {
                    PlayerInput::Number(n) => return Ok(n.into()),
                    _ => return Err("Expected numeric input".into()),
                }
            }
            Function::InputString => {
                let mut packet = PacketZcOpenEditdlgstr::new(packetver);
                packet.naid = npc;
                packet.fill_raw();
                self.send_packet(&mut packet).await?;
                match self.receive().await? {
                    PlayerInput::Text(s) => return Ok(s.into()),
                    _ => return Err("Expected text input".into()),
                }
            }
            Function::Message | Function::DispBottom => {
                let text = arguments.last().ok_or("Missing message")?.text();
                if text.len() > 16_000 {
                    return Err("Message is too long".into());
                }
                let mut packet = playerchat_packet(packetver, &text);
                self.send_packet(&mut packet).await?;
            }
            Function::Cutin => {
                let file = arguments.first().ok_or("Missing image")?.text();
                let position = arguments.get(1).ok_or("Missing image position")?.number_value()?;
                let mut name = [char::from(0); 64];
                for (slot, value) in name.iter_mut().zip(file.chars()) {
                    *slot = value;
                }
                let mut packet = PacketZcShowImage2::new(packetver);
                packet.set_image_name(name);
                packet.set_atype(u8::try_from(position).map_err(|_| "Invalid image position")?);
                packet.fill_raw();
                self.send_packet(&mut packet).await?;
            }
            _ => return Err("Unknown interaction".into()),
        }
        Ok(Value::default())
    }
}
