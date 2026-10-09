use crate::ctx::Ctx;
use crate::flow::{Script, Stop};

impl<'a> Ctx<'a> {
    /// The player in this conversation, such as `ctx.player().zeny()`.
    pub fn player<'c>(&'c self) -> Player<'c, 'a> {
        Player { ctx: self }
    }
}

/// Status of the player in this conversation. Reads go through the character's status variables.
pub struct Player<'c, 'a> {
    ctx: &'c Ctx<'a>,
}

impl Player<'_, '_> {
    pub fn name(&self) -> Result<String, Stop> {
        self.char_info(0)
    }

    pub fn party_name(&self) -> Result<String, Stop> {
        self.char_info(1)
    }

    pub fn guild_name(&self) -> Result<String, Stop> {
        self.char_info(2)
    }

    /// The map the player is on, such as `"prontera"`.
    pub fn map_name(&self) -> Result<String, Stop> {
        self.char_info(3)
    }

    fn char_info(&self, field: i32) -> Result<String, Stop> {
        self.ctx.call(script_sdk::Function::StrCharInfo, crate::args![field]).map(|value| value.text())
    }

    pub fn zeny(&self) -> Result<i32, Stop> {
        self.ctx.var("Zeny").get()?.number()
    }

    pub fn set_zeny(&self, amount: i32) -> Script {
        self.ctx.var("Zeny").set(amount)
    }

    pub fn base_level(&self) -> Result<i32, Stop> {
        self.ctx.var("BaseLevel").get()?.number()
    }

    pub fn job_level(&self) -> Result<i32, Stop> {
        self.ctx.var("JobLevel").get()?.number()
    }

    /// The job id, such as `Class` in rathena scripts.
    pub fn class(&self) -> Result<i32, Stop> {
        self.ctx.var("Class").get()?.number()
    }
}

#[cfg(test)]
mod tests {
    use script_sdk::{Request, Value};

    use crate::transport::MockTransport;
    use crate::Ctx;

    #[test]
    fn zeny_reads_the_character_status_variable() {
        let transport = MockTransport::new(|request| match request {
            Request::Read(name) if name == "Zeny" => Ok(Value::Number(500)),
            _ => Ok(Value::default()),
        });
        assert_eq!(Ctx::new(&transport).player().zeny(), Ok(500));
    }
}
