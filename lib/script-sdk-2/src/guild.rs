use script_sdk::Function;

use crate::args;
use crate::ctx::Ctx;
use crate::flow::{Script, Stop};
use crate::value::Val;

impl<'a> Ctx<'a> {
    /// Guild lookups and rewards, such as `ctx.guild().name(guild_id)`.
    pub fn guild<'c>(&'c self) -> Guild<'c, 'a> {
        Guild { ctx: self }
    }
}

pub struct Guild<'c, 'a> {
    ctx: &'c Ctx<'a>,
}

impl Guild<'_, '_> {
    /// The name of guild `guild_id`, or empty when the guild does not exist.
    pub fn name(&self, guild_id: i32) -> Result<String, Stop> {
        self.ctx.call(Function::GetGuildInfo, args![guild_id, 0]).map(|value| value.text())
    }

    /// The name of the guild master, or `"null"` when it is not known.
    pub fn master_name(&self, guild_id: i32) -> Result<String, Stop> {
        self.ctx.call(Function::GetGuildMaster, args![guild_id]).map(|value| value.text())
    }

    /// Whether the attached player is the master of guild `guild_id`.
    pub fn is_master(&self, guild_id: i32) -> Result<bool, Stop> {
        Ok(self.ctx.call(Function::GetGuildInfo, args![guild_id, 2])?.number()? != 0)
    }

    /// The level of guild skill `skill` in guild `guild_id`. `-1` when the guild does not exist.
    pub fn skill_level(&self, guild_id: i32, skill: i32) -> Result<i32, Stop> {
        self.ctx.call(Function::GetGuildSkillLevel, args![guild_id, skill])?.number()
    }

    /// Adds `amount` to the guild experience of the attached player's guild. Errors when `amount` is not positive.
    pub fn experience(&self, amount: i32) -> Script {
        self.ctx.call(Function::GuildExperience, args![amount]).map(|_| ())
    }

    /// Opens the guild storage for the attached player. Returns whether it opened.
    pub fn open_storage(&self) -> Result<bool, Stop> {
        Ok(self.ctx.call(Function::GuildOpenStorage, Vec::<Val>::new())?.number()? != 0)
    }
}

#[cfg(test)]
mod tests {
    use script_sdk::{Function, Value};

    use crate::transport::MockTransport;
    use crate::Ctx;

    #[test]
    fn is_master_asks_for_the_master_flag() {
        let transport = MockTransport::new(|_| Ok(Value::Number(1)));
        assert_eq!(Ctx::new(&transport).guild().is_master(5), Ok(true));
        assert_eq!(transport.calls(Function::GetGuildInfo)[0], vec![Value::new_number(5), Value::new_number(2)]);
    }

    #[test]
    fn name_asks_for_the_name_field() {
        let transport = MockTransport::new(|_| Ok(Value::new_string("Rust".into())));
        assert_eq!(Ctx::new(&transport).guild().name(5).as_deref(), Ok("Rust"));
        assert_eq!(transport.calls(Function::GetGuildInfo)[0][1], Value::new_number(0));
    }
}
