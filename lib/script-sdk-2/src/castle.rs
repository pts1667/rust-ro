use script_sdk::Function;

use crate::args;
use crate::ctx::Ctx;
use crate::flow::{Script, Stop};

impl<'a> Ctx<'a> {
    /// The War of Emperium castles, such as `ctx.castle().data("prtg_cas01", 1)`.
    pub fn castle<'c>(&'c self) -> Castle<'c, 'a> {
        Castle { ctx: self }
    }
}

pub struct Castle<'c, 'a> {
    ctx: &'c Ctx<'a>,
}

impl Castle<'_, '_> {
    /// A field of the castle on `map`, as rathena's `getcastledata`: `1` owning guild, `2` economy, `3` defense,
    /// `4` and `5` today's investments in them, `9` Kafra hired, `10` onwards the guardian slots.
    pub fn data(&self, map: &str, field: i32) -> Result<i32, Stop> {
        self.ctx.call(Function::GetCastleData, args![map, field])?.number()
    }

    pub fn set_data(&self, map: &str, field: i32, value: i32) -> Script {
        self.ctx.call(Function::SetCastleData, args![map, field, value]).map(|_| ())
    }

    /// Spawns the guardian of slot `slot` (from 0) of the castle on `map`.
    pub fn summon_guardian(&self, map: &str, slot: i32) -> Script {
        self.ctx.call(Function::GuardianSummon, args![map, slot]).map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use script_sdk::{Function, Value};

    use crate::transport::MockTransport;
    use crate::Ctx;

    #[test]
    fn set_data_sends_the_map_field_and_value() {
        let transport = MockTransport::silent();
        Ctx::new(&transport).castle().set_data("prtg_cas01", 9, 1).unwrap();
        assert_eq!(transport.calls(Function::SetCastleData)[0], vec![Value::new_string("prtg_cas01".into()), Value::new_number(9), Value::new_number(1)]);
    }
}
