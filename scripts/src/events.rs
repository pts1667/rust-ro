use script_sdk::{Context, Request, Value, Variable, VariableScope};

pub fn run(ctx: &Context, id: u32) -> Result<(), String> {
    match id {
        1 => {
            let Value::Array(arguments) = ctx.request(Request::Arguments)? else { return Err("Event arguments are invalid".into()); };
            if arguments.len() < 2 { return Err("Monster death event needs its runtime and class IDs".into()); }
            ctx.request(Request::VariablesIncrement(vec![Variable { scope: VariableScope::Character, name: "SummonedMobKills".into(), index: 0, value: 1.into() }]))?;
            Ok(())
        }
        _ => Err(format!("Unknown compiled event {id}")),
    }
}
