use script_sdk::{Context, Request, Value, Variable, VariableScope};

pub fn run(ctx: &Context, id: u32) -> Result<(), String> {
    match id {
        1 => {
            let Value::Array(arguments) = ctx.request(Request::Arguments)? else { return Err("Event arguments are invalid".into()); };
            if arguments.len() < 2 { return Err("Monster death event needs its runtime and class IDs".into()); }
            ctx.request(Request::VariablesIncrement(vec![Variable { scope: VariableScope::Character, name: "SummonedMobKills".into(), index: 0, value: 1.into() }]))?;
            Ok(())
        }
        3000..=3999 => crate::battleground_tierra::event(ctx, ((id - 3000) / 100) as usize, (id - 3000) % 100),
        2000..=2999 => crate::battleground_kvm::event(ctx, ((id - 2000) / 100) as usize, (id - 2000) % 100),
        1000..=1999 => crate::battleground_arena::event(ctx, ((id - 1000) / 100) as usize, (id - 1000) % 100),
        _ => Err(format!("Unknown compiled event {id}")),
    }
}
