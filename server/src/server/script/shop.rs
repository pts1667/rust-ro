use std::collections::HashMap;

use models::enums::skill_enums::SkillEnum;
use models::enums::EnumWithNumberValue;
use packets::packets::{
    Packet, PacketZcPcPurchaseItemlist, PacketZcPcPurchaseResult, PacketZcPcSellItemlist, PacketZcSelectDealtype, PurchaseItem, SellItem,
};
use script_sdk::{Function, Reply, Request, Value};

use super::{NpcScriptHost, PlayerInput};
use crate::server::service::global_config_service::GlobalConfigService;

/// `min_shop_buy` of rathena.
const MIN_SHOP_BUY: i32 = 1;

pub(crate) fn skill_ids() -> [i32; 3] {
    [SkillEnum::McDiscount.id() as i32, SkillEnum::RgCompulsion.id() as i32, SkillEnum::McOvercharge.id() as i32]
}

/// `pc_modifybuyvalue`: Discount or Compulsion Discount, whichever is better.
pub(crate) fn discounted_price(price: i32, discount: i32, compulsion: i32) -> i32 {
    let merchant = if discount > 0 { 5 + discount * 2 - i32::from(discount == 10) } else { 0 };
    let rogue = if compulsion > 0 { 5 + compulsion * 4 } else { 0 };
    let rate = merchant.max(rogue);
    let value = if rate > 0 { (f64::from(price) * f64::from(100 - rate) / 100.0) as i32 } else { price };
    value.max(MIN_SHOP_BUY)
}

/// `pc_modifysellvalue`: Overcharge.
pub(crate) fn overcharged_price(price: i32, overcharge: i32) -> i32 {
    if overcharge <= 0 {
        return price;
    }
    let rate = 5 + overcharge * 2 - i32::from(overcharge == 10);
    (f64::from(price) * f64::from(100 + rate) / 100.0) as i32
}

impl NpcScriptHost {
    async fn skill_levels(&self) -> Result<[i32; 3], String> {
        let mut levels = [0; 3];
        for (level, id) in levels.iter_mut().zip(skill_ids()) {
            *level = self.forward(Request::Call { function: Function::GetSkillLv, arguments: vec![Value::Number(id)] }).await?.number_value()?;
        }
        Ok(levels)
    }

    pub async fn shop(&mut self) -> Reply {
        let packetver = self.server.packetver();
        let mut dealer = PacketZcSelectDealtype::new(packetver);
        dealer.naid = self.script.id;
        dealer.fill_raw();
        self.send_packet(&mut dealer).await?;
        let reply = match self.receive().await? {
            PlayerInput::DealType(0) => self.purchase().await,
            PlayerInput::DealType(1) => self.sell().await,
            _ => Err("Shop cancelled".into()),
        };
        self.interaction(Function::Close, vec![]).await?;
        reply
    }

    async fn purchase(&mut self) -> Reply {
        let packetver = self.server.packetver();
        let arguments = &self.script.constructor_args;
        if arguments.len() < 3 || arguments.len() % 2 == 0 {
            return Err("Invalid shop configuration".into());
        }
        let [discount, compulsion, _] = self.skill_levels().await?;
        let mut offers = HashMap::new();
        let mut entries = vec![];
        for pair in arguments[1..].chunks_exact(2) {
            let id = pair[0].number_value()?;
            let item = GlobalConfigService::instance().find_item(id).ok_or("Unknown shop item")?;
            let override_price = pair[1].number_value()?;
            let price = if override_price == -1 {
                item.price_buy.unwrap_or(0)
            } else {
                override_price
            };
            if price < 0 {
                return Err("Invalid shop price".into());
            }
            let mut entry = PurchaseItem::new(packetver);
            entry.set_itid(u16::try_from(id).map_err(|_| "Shop item ID is too large")?);
            entry.set_atype(item.item_type.value() as u8);
            entry.set_price(price);
            entry.set_discountprice(discounted_price(price, discount, compulsion));
            entries.push(entry);
            offers.insert(id as u32, price);
        }
        let mut packet = PacketZcPcPurchaseItemlist::new(packetver);
        packet.set_packet_length(
            (PacketZcPcPurchaseItemlist::base_len(packetver) + entries.len() * PurchaseItem::base_len(packetver)) as i16,
        );
        packet.set_item_list(entries);
        packet.fill_raw();
        self.send_packet(&mut packet).await?;
        let PlayerInput::Purchases(items) = self.receive().await? else {
            return Err("Expected shop purchase".into());
        };
        let mut merged: HashMap<u32, i16> = HashMap::new();
        for (id, amount) in items {
            if amount <= 0 || !offers.contains_key(&id) {
                return Err("Invalid shop purchase".into());
            }
            let total = merged.entry(id).or_default();
            *total = total.checked_add(amount).ok_or("Purchase amount overflow")?;
        }
        if merged.is_empty() {
            return Err("Empty purchase".into());
        }
        let result = self
            .forward(Request::Purchase(
                merged.into_iter().map(|(id, amount)| (id, amount, offers[&id])).collect(),
            ))
            .await;
        if result.as_ref().is_err_and(|error| error == "You cannot hold this amount of items") {
            let mut packet = PacketZcPcPurchaseResult::new(packetver);
            packet.set_result(2);
            packet.fill_raw();
            self.send_packet(&mut packet).await?;
        }
        result
    }

    async fn sell(&mut self) -> Reply {
        let packetver = self.server.packetver();
        let Value::Array(inventory) = self.forward(Request::Inventory).await? else {
            return Err("Invalid inventory snapshot".into());
        };
        let [_, _, overcharge] = self.skill_levels().await?;
        let mut entries = vec![];
        let mut prices = HashMap::new();
        for row in inventory {
            let Value::Array(values) = row else {
                return Err("Invalid inventory row".into());
            };
            if values.len() != 4 {
                return Err("Invalid inventory row".into());
            }
            let index = values[0].number_value()?;
            let price = values[3].number_value()?;
            let mut entry = SellItem::new(packetver);
            entry.set_index(i16::try_from(index).map_err(|_| "Invalid inventory index")?);
            entry.set_price(price);
            entry.set_overchargeprice(overcharged_price(price, overcharge));
            entries.push(entry);
            prices.insert(index as usize, price);
        }
        let mut packet = PacketZcPcSellItemlist::new(packetver);
        packet.set_packet_length((PacketZcPcSellItemlist::base_len(packetver) + entries.len() * SellItem::base_len(packetver)) as i16);
        packet.set_item_list(entries);
        packet.fill_raw();
        self.send_packet(&mut packet).await?;
        let PlayerInput::Sales(items) = self.receive().await? else {
            return Err("Expected shop sale".into());
        };
        let mut merged: HashMap<usize, i16> = HashMap::new();
        for (index, amount) in items {
            if amount <= 0 || !prices.contains_key(&index) {
                return Err("Invalid shop sale".into());
            }
            let total = merged.entry(index).or_default();
            *total = total.checked_add(amount).ok_or("Sale amount overflow")?;
        }
        if merged.is_empty() {
            return Err("Empty sale".into());
        }
        self.forward(Request::Sale(
            merged.into_iter().map(|(index, amount)| (index, amount, prices[&index])).collect(),
        ))
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::{discounted_price, overcharged_price};

    #[test]
    fn merchant_skills_change_shop_prices() {
        assert_eq!(discounted_price(1000, 10, 0), 760);
        assert_eq!(discounted_price(1000, 5, 5), 750);
        assert_eq!(discounted_price(0, 0, 0), 1);
        assert_eq!(overcharged_price(1000, 10), 1240);
        assert_eq!(overcharged_price(1000, 0), 1000);
    }
}
