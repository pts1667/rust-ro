//! Example plugin of custom items and `@` commands (`docs/plugins.md`). Its items are in
//! `config/plugins/lucky_charms/items.json`: a Lucky Ticket whose use draws a reward, a Clover Charm accessory whose bonus
//! script adds luck, and a Fortune Cookie whose use script talks. `@lucky` counts a player's tickets and `@luckytickets`
//! hands some out to staff. Settings and their defaults are in `info.json`.

use script_sdk_2::{Bonus, Ctx, Function, ItemBonus, ItemUse, PluginCommand, Script, Stop};
use serde::Deserialize;

script_sdk_2::plugin_module! {
    info { name: "lucky_charms", version: "1.0.0", capabilities: [Dialogue, Economy, Character] }
    init => init,
    npcs {}
    events {}
    hooks {}
    interceptors {}
    commands {
        "lucky" => (count_tickets, 0, "Tells how many Lucky Tickets you hold"),
        "luckytickets" => (give_tickets, 60, "Gives you Lucky Tickets: @luckytickets [amount]"),
    }
    items {
        40001 => lucky_ticket,
        40003 => fortune_cookie,
    }
    item_bonuses { 40002 => clover_charm }
}

const LUCKY_TICKET: i32 = 40001;

#[derive(Deserialize)]
struct Settings {
    rewards: Vec<Reward>,
    charm_luk: i32,
    fortunes: Vec<String>,
    max_given: i32,
}

#[derive(Deserialize, Clone, Copy)]
struct Reward {
    item_id: i32,
    amount: i32,
    weight: i32,
}

fn init(ctx: &Ctx) -> Script {
    let settings: Settings = ctx.plugin_config()?;
    if settings.rewards.is_empty() || settings.rewards.iter().any(|reward| reward.weight <= 0 || reward.amount <= 0) {
        return Err(Stop::from("Every Lucky Ticket reward needs a positive amount and weight"));
    }
    if settings.fortunes.is_empty() {
        return Err(Stop::from("A Fortune Cookie needs at least one fortune"));
    }
    Ok(())
}

/// Item scripts run outside `Ctx`, so they read the settings through the call itself.
fn settings(call: impl Fn(Function, Vec<script_sdk_2::Val>) -> Result<script_sdk_2::Val, Stop>) -> Result<Settings, Stop> {
    let text = call(Function::PluginConfig, vec![])?.text();
    serde_json::from_str(&text).map_err(|error| Stop::from(format!("Invalid plugin configuration: {error}")))
}

/// The reward of a draw of `roll` in `0..total weight`.
fn reward_for(rewards: &[Reward], roll: i32) -> Option<Reward> {
    let mut remaining = roll;
    rewards.iter().copied().find(|reward| {
        remaining -= reward.weight;
        remaining < 0
    })
}

fn lucky_ticket(item: &ItemUse) -> Script {
    let settings = settings(|function, arguments| item.call(function, arguments))?;
    let total: i32 = settings.rewards.iter().map(|reward| reward.weight).sum();
    let roll = item.call(Function::Rand, vec![total.into()])?.number()?;
    let reward = reward_for(&settings.rewards, roll).ok_or_else(|| Stop::from("The draw found no reward"))?;
    item.grant(reward.item_id, reward.amount)
}

fn fortune_cookie(item: &ItemUse) -> Script {
    let settings = settings(|function, arguments| item.call(function, arguments))?;
    let index = item.call(Function::Rand, vec![(settings.fortunes.len() as i32).into()])?.number()?;
    let fortune = settings.fortunes.get(index as usize).ok_or_else(|| Stop::from("No fortune"))?;
    item.heal_percent(5, 5)?;
    item.mes_as("Fortune Cookie", fortune)?;
    item.close()
}

fn clover_charm(item: &ItemBonus) -> Script {
    let settings = settings(|function, arguments| item.call(function, arguments))?;
    item.bonus().apply(Bonus::Luk(settings.charm_luk))
}

fn count_tickets(ctx: &Ctx, _command: &PluginCommand) -> Script {
    let count = ctx.items().count(LUCKY_TICKET)?;
    ctx.reply(&format!("You hold {count} Lucky Ticket{}.", if count == 1 { "" } else { "s" }))
}

fn give_tickets(ctx: &Ctx, command: &PluginCommand) -> Script {
    let settings: Settings = ctx.plugin_config()?;
    let amount = match command.args.first() {
        None => 1,
        Some(word) => match word.parse::<i32>() {
            Ok(amount) if (1..=settings.max_given).contains(&amount) => amount,
            _ => return ctx.reply(&format!("Usage: @luckytickets [amount], the amount from 1 to {}.", settings.max_given)),
        },
    };
    ctx.items().give(LUCKY_TICKET, amount)?;
    ctx.reply(&format!("Here are {amount} Lucky Tickets."))
}

#[cfg(test)]
mod tests {
    use script_sdk_2::{MockTransport, Request, Value};

    use super::*;

    fn transport(rand: i32, tickets: i32) -> MockTransport {
        let config = serde_json::json!({
            "rewards": [{ "item_id": 501, "amount": 5, "weight": 50 }, { "item_id": 607, "amount": 1, "weight": 5 }],
            "charm_luk": 3,
            "fortunes": ["Good luck."],
            "max_given": 10,
        })
        .to_string();
        MockTransport::new(move |request| match request {
            Request::Call { function: Function::PluginConfig, .. } => Ok(Value::String(config.clone())),
            Request::Call { function: Function::Rand, .. } => Ok(Value::Number(rand)),
            Request::Call { function: Function::CountItem, .. } => Ok(Value::Number(tickets)),
            _ => Ok(Value::default()),
        })
    }

    #[test]
    fn a_draw_lands_on_the_reward_whose_weight_covers_the_roll() {
        let rewards = [Reward { item_id: 1, amount: 1, weight: 50 }, Reward { item_id: 2, amount: 1, weight: 5 }];
        assert_eq!(reward_for(&rewards, 0).map(|reward| reward.item_id), Some(1));
        assert_eq!(reward_for(&rewards, 49).map(|reward| reward.item_id), Some(1));
        assert_eq!(reward_for(&rewards, 50).map(|reward| reward.item_id), Some(2));
        assert!(reward_for(&rewards, 55).is_none());
    }

    #[test]
    fn a_lucky_ticket_grants_the_reward_it_draws() {
        let transport = transport(52, 0);
        let ctx = Ctx::new(&transport);
        lucky_ticket(&ItemUse::new(&ctx)).unwrap();
        let grants = transport.calls(Function::GetItem);
        assert_eq!(grants, [vec![Value::Number(607), Value::Number(1)]]);
    }

    #[test]
    fn the_clover_charm_adds_the_configured_luck() {
        let transport = transport(0, 0);
        let ctx = Ctx::new(&transport);
        clover_charm(&ItemBonus::new(&ctx)).unwrap();
        assert_eq!(transport.calls(Function::Bonus).len(), 1);
    }

    #[test]
    fn commands_answer_in_the_chat_window() {
        let transport = transport(0, 1);
        let ctx = Ctx::new(&transport);
        count_tickets(&ctx, &PluginCommand { name: "lucky".into(), args: vec![] }).unwrap();
        give_tickets(&ctx, &PluginCommand { name: "luckytickets".into(), args: vec!["99".into()] }).unwrap();
        give_tickets(&ctx, &PluginCommand { name: "luckytickets".into(), args: vec!["3".into()] }).unwrap();
        let replies: Vec<String> = transport.calls(Function::CommandReply).into_iter().map(|arguments| arguments[0].text()).collect();
        assert_eq!(replies, ["You hold 1 Lucky Ticket.", "Usage: @luckytickets [amount], the amount from 1 to 10.", "Here are 3 Lucky Tickets."]);
        assert_eq!(transport.calls(Function::GetItem), [vec![Value::Number(LUCKY_TICKET), Value::Number(3)]]);
    }
}
