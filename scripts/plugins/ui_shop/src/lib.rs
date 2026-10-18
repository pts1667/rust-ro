//! Example plugin of custom UI (`docs/plugin-ui.md`): the Curio Trader in Prontera opens a shop window with an animated
//! coin, a scrolling table of items and an amount input, a level up shows a notification above the player, and bots can
//! read the stock and buy through a plugin action. Settings are in `config/plugins/ui_shop/config.json`.

use script_sdk_2::ui::{Align, Easing, EventKind, ImageRef, Node, Overhead, Table, UiEvent, Window};
use script_sdk_2::{Above, BotAction, Ctx, Hook, LevelKind, Script, Stop, bot_args};
use serde::{Deserialize, Serialize};

script_sdk_2::plugin_module! {
    info { name: "ui_shop", version: "1.0.0", capabilities: [Interface, Economy] }
    init => init,
    npcs { "Curio Trader" => curio_trader }
    events {}
    hooks { LevelUp => level_up }
    interceptors {}
    windows { "shop" => shop_event }
    bot_actions { "buy" => (bot_buy, "Buys from the Curio Trader while its window is open: {item_id, amount}") }
}

const WINDOW: &str = "shop";
/// Set while the shop window is open, so bot purchases need the trader to have been visited.
const SHOP_OPEN: &str = "@ui_shop_open";

#[derive(Deserialize)]
#[serde(default)]
struct Settings {
    title: String,
    greeting: String,
    max_amount: i32,
    level_up_overhead: bool,
    stock: Vec<Offer>,
}

impl Default for Settings {
    fn default() -> Self {
        Self { title: "Curio Trader".into(), greeting: String::new(), max_amount: 100, level_up_overhead: true, stock: Vec::new() }
    }
}

#[derive(Deserialize, Clone, Copy)]
struct Offer {
    item_id: i32,
    price: i32,
}

/// One line of the stock as bots observe it.
#[derive(Serialize)]
struct StockLine {
    item_id: i32,
    name: String,
    price: i32,
}

#[derive(Deserialize)]
struct BuyArgs {
    item_id: i32,
    #[serde(default = "one")]
    amount: i32,
}

fn one() -> i32 {
    1
}

fn init(ctx: &Ctx) -> Script {
    let settings: Settings = ctx.plugin_config()?;
    if settings.stock.iter().any(|offer| offer.price <= 0) {
        return Err(Stop::from("Every offer of the Curio Trader needs a positive price"));
    }
    Ok(())
}

fn curio_trader(ctx: &Ctx) -> Script {
    let settings: Settings = ctx.plugin_config()?;
    let stock = stock_lines(ctx, &settings)?;
    ctx.var(SHOP_OPEN).set(1)?;
    ctx.bot().observe("stock", &stock)?;
    ctx.ui().open(&shop_window(ctx, &settings, &stock, None)?)
}

fn stock_lines(ctx: &Ctx, settings: &Settings) -> Result<Vec<StockLine>, Stop> {
    settings.stock.iter().map(|offer| Ok(StockLine { item_id: offer.item_id, name: ctx.items().name(offer.item_id)?, price: offer.price })).collect()
}

/// `notice` is the outcome of the last purchase, green when it went through.
fn shop_window(ctx: &Ctx, settings: &Settings, stock: &[StockLine], notice: Option<Result<String, String>>) -> Result<Window, Stop> {
    let zeny = ctx.player().zeny()?;
    let mut table = Table::new("stock", 6)
        .column("", 32, Align::Center)
        .column("Item", 150, Align::Start)
        .column("Price", 70, Align::End)
        .column("", 56, Align::Center)
        .row_height(28);
    for line in stock {
        let buy = Node::button(format!("buy:{}", line.item_id), "Buy");
        table = table.row(
            line.item_id.to_string(),
            [Node::item(line.item_id as u32), Node::text(&line.name), Node::text(format!("{} z", thousands(line.price))), if zeny < line.price { buy.disabled() } else { buy }],
        );
    }
    let mut children = vec![
        Node::row([Node::image(ImageRef::plugin_animated(["coin_0.png", "coin_1.png", "coin_2.png", "coin_3.png"], 8.0), 24, 24), Node::title(format!("{} zeny", thousands(zeny)))]).gap(8),
        Node::wrapped(&settings.greeting),
        table.into(),
        Node::row([Node::text("Amount"), Node::number_input("amount", 1, 1, settings.max_amount.max(1)), Node::spacer(90), Node::button("close", "Close")]),
    ];
    match notice {
        Some(Ok(text)) => children.push(Node::text(text).color("00a000")),
        Some(Err(text)) => children.push(Node::text(text).color("d02020")),
        None => {}
    }
    Ok(Window::new(WINDOW, &settings.title, 340, Node::column(children).gap(6)))
}

fn thousands(value: i32) -> String {
    let digits = value.unsigned_abs().to_string();
    let mut grouped = String::new();
    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index) % 3 == 0 {
            grouped.push(',');
        }
        grouped.push(digit);
    }
    if value < 0 { format!("-{grouped}") } else { grouped }
}

fn shop_event(ctx: &Ctx, event: &UiEvent) -> Script {
    match (event.kind, event.widget.as_str()) {
        (EventKind::Closed, _) => leave(ctx),
        (EventKind::Click, "close") => {
            leave(ctx)?;
            ctx.ui().close(WINDOW)
        }
        (EventKind::Click, widget) => {
            let Some(item_id) = widget.strip_prefix("buy:").and_then(|id| id.parse().ok()) else { return Ok(()) };
            let settings: Settings = ctx.plugin_config()?;
            let outcome = buy(ctx, &settings, item_id, event.number("amount").unwrap_or(1));
            let stock = stock_lines(ctx, &settings)?;
            ctx.ui().open(&shop_window(ctx, &settings, &stock, Some(outcome.map(|bought| bought.to_string())))?)
        }
        _ => Ok(()),
    }
}

fn leave(ctx: &Ctx) -> Script {
    ctx.var(SHOP_OPEN).set(0)?;
    ctx.bot().forget("stock")
}

struct Bought {
    amount: i32,
    name: String,
    zeny: i32,
}

impl std::fmt::Display for Bought {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "Bought {} {}.", self.amount, self.name)
    }
}

/// `Err` carries what to tell the buyer; a failed host call stops the script instead.
fn buy(ctx: &Ctx, settings: &Settings, item_id: i32, amount: i32) -> Result<Bought, String> {
    let fail = |stop: Stop| format!("The trade failed: {stop:?}");
    let offer = settings.stock.iter().find(|offer| offer.item_id == item_id).ok_or("The trader does not sell that.")?;
    if amount < 1 || amount > settings.max_amount {
        return Err(format!("Buy between 1 and {} at a time.", settings.max_amount));
    }
    let total = i64::from(offer.price) * i64::from(amount);
    let zeny = ctx.player().zeny().map_err(fail)?;
    if total > i64::from(zeny) {
        return Err(format!("That costs {} zeny.", thousands(i32::try_from(total).unwrap_or(i32::MAX))));
    }
    if !ctx.items().check_weight(&[(item_id, amount)]).map_err(fail)? {
        return Err("You cannot carry that much.".into());
    }
    let zeny = zeny - total as i32;
    ctx.player().set_zeny(zeny).map_err(fail)?;
    ctx.items().give(item_id, amount).map_err(fail)?;
    Ok(Bought { amount, name: ctx.items().name(item_id).map_err(fail)?, zeny })
}

fn bot_buy(ctx: &Ctx, action: &BotAction) -> Script {
    let args: BuyArgs = bot_args(action)?;
    if ctx.var(SHOP_OPEN).get()?.number()? == 0 {
        return ctx.bot().reply(&serde_json::json!({ "bought": 0, "error": "Talk to the Curio Trader first" }));
    }
    let settings: Settings = ctx.plugin_config()?;
    match buy(ctx, &settings, args.item_id, args.amount) {
        Ok(bought) => {
            let stock = stock_lines(ctx, &settings)?;
            ctx.ui().open(&shop_window(ctx, &settings, &stock, Some(Ok(bought.to_string())))?)?;
            ctx.bot().reply(&serde_json::json!({ "bought": bought.amount, "item_id": args.item_id, "zeny": bought.zeny }))
        }
        Err(error) => ctx.bot().reply(&serde_json::json!({ "bought": 0, "error": error })),
    }
}

fn level_up(ctx: &Ctx, hook: &Hook) -> Script {
    let Hook::LevelUp { kind, level, .. } = hook else { return Ok(()) };
    let settings: Settings = ctx.plugin_config()?;
    if !settings.level_up_overhead {
        return Ok(());
    }
    let (caption, color) = match kind {
        LevelKind::Base => (format!("LEVEL {level}!"), "ffd040"),
        LevelKind::Job => (format!("JOB LEVEL {level}!"), "80c0ff"),
    };
    let overhead = Overhead::image(ImageRef::plugin("star.png"), 48, 48, 2200)
        .caption(caption, Some(color))
        .offset(10)
        .pop(350)
        .rise(36.0, 1600)
        .easing(Easing::EaseOut)
        .fade_in(150)
        .fade_out(600);
    ctx.ui().overhead(Above::Player, &overhead)
}

#[cfg(test)]
mod tests {
    use script_sdk_2::{Function, MockTransport, Request, Value};

    use super::*;

    fn settings() -> Settings {
        Settings { stock: vec![Offer { item_id: 501, price: 40 }, Offer { item_id: 1750, price: 1 }], ..Settings::default() }
    }

    fn host(zeny: i32, can_carry: bool) -> MockTransport {
        MockTransport::new(move |request| match request {
            Request::Read(name) if name == "Zeny" => Ok(Value::Number(zeny)),
            Request::Call { function: Function::CheckWeight, .. } => Ok(Value::Number(i32::from(can_carry))),
            Request::Call { function: Function::GetItemName, .. } => Ok(Value::String("Red Potion".into())),
            _ => Ok(Value::default()),
        })
    }

    #[test]
    fn thousands_are_grouped() {
        assert_eq!(thousands(0), "0");
        assert_eq!(thousands(999), "999");
        assert_eq!(thousands(1_000), "1,000");
        assert_eq!(thousands(1_234_567), "1,234,567");
        assert_eq!(thousands(-4_000), "-4,000");
    }

    #[test]
    fn a_purchase_takes_the_zeny_and_gives_the_items() {
        let transport = host(100, true);
        let bought = buy(&Ctx::new(&transport), &settings(), 501, 2).unwrap();
        assert_eq!((bought.amount, bought.zeny), (2, 20));
        let requests = transport.requests();
        assert!(requests.contains(&Request::Write { name: "Zeny".into(), value: Value::Number(20) }), "{requests:?}");
        assert!(requests.iter().any(|request| matches!(request, Request::Call { function: Function::GetItem, arguments } if arguments[1] == Value::Number(2))));
    }

    #[test]
    fn purchases_the_player_cannot_afford_or_carry_are_refused_before_anything_changes() {
        for (transport, item, amount, reason) in [
            (host(50, true), 501, 2, "costs 80"),
            (host(1000, false), 501, 2, "carry"),
            (host(1000, true), 999, 1, "does not sell"),
            (host(1000, true), 501, 101, "between 1 and 100"),
        ] {
            let error = buy(&Ctx::new(&transport), &settings(), item, amount).err().unwrap();
            assert!(error.contains(reason), "{error}");
            assert!(!transport.requests().iter().any(|request| matches!(request, Request::Write { name, .. } if name == "Zeny")));
        }
    }

    #[test]
    fn the_window_disables_what_the_player_cannot_afford() {
        let transport = host(20, true);
        let ctx = Ctx::new(&transport);
        let stock = stock_lines(&ctx, &settings()).unwrap();
        let mut window = shop_window(&ctx, &settings(), &stock, Some(Err("No.".into()))).unwrap();
        assert!(matches!(window.widget("buy:501"), Some(Node::Button { enabled: false, .. })));
        assert!(matches!(window.widget("buy:1750"), Some(Node::Button { enabled: true, .. })));
        assert_eq!(script_sdk_2::ui::validate_window(&mut window, "ui_shop"), Ok(()));
    }
}
