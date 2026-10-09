use script_sdk::Function;

use crate::args;
use crate::ctx::Ctx;
use crate::flow::Stop;

impl<'a> Ctx<'a> {
    /// Party checks for the player, such as `ctx.party().is_leader(party_id)`.
    pub fn party<'c>(&'c self) -> Party<'c, 'a> {
        Party { ctx: self }
    }
}

pub struct Party<'c, 'a> {
    ctx: &'c Ctx<'a>,
}

impl Party<'_, '_> {
    /// Whether the player leads party `party_id`.
    pub fn is_leader(&self, party_id: i32) -> Result<bool, Stop> {
        Ok(self.ctx.call(Function::IsPartyLeader, args![party_id])?.number()? != 0)
    }
}

impl Party<'_, '_> {
    /// The name of party `party_id`, or empty when no online member belongs to it.
    pub fn name(&self, party_id: i32) -> Result<String, Stop> {
        self.ctx.call(Function::GetPartyName, args![party_id]).map(|value| value.text())
    }
}

#[cfg(test)]
mod tests {
    use script_sdk::{Function, Value};

    use crate::transport::MockTransport;
    use crate::Ctx;

    #[test]
    fn is_leader_sends_the_party_id() {
        let transport = MockTransport::new(|_| Ok(Value::Number(0)));
        assert_eq!(Ctx::new(&transport).party().is_leader(7), Ok(false));
        assert_eq!(transport.calls(Function::IsPartyLeader), vec![vec![Value::new_number(7)]]);
    }

    #[test]
    fn name_sends_the_party_id() {
        let transport = MockTransport::new(|_| Ok(Value::new_string("Rust".into())));
        assert_eq!(Ctx::new(&transport).party().name(7).as_deref(), Ok("Rust"));
        assert_eq!(transport.calls(Function::GetPartyName), vec![vec![Value::new_number(7)]]);
    }
}
