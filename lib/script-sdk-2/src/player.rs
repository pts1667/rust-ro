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

impl Player<'_, '_> {
    /// The character id of the attached player.
    pub fn char_id(&self) -> Result<i32, Stop> {
        self.id_of(0)
    }

    pub fn party_id(&self) -> Result<i32, Stop> {
        self.id_of(1)
    }

    pub fn guild_id(&self) -> Result<i32, Stop> {
        self.id_of(2)
    }

    pub fn account_id(&self) -> Result<i32, Stop> {
        self.id_of(3)
    }

    /// The battleground team of the attached player, `0` outside a battleground.
    pub fn battleground_id(&self) -> Result<i32, Stop> {
        self.id_of(4)
    }

    fn id_of(&self, kind: i32) -> Result<i32, Stop> {
        self.ctx.call(script_sdk::Function::GetCharacterId, crate::args![kind])?.number()
    }

    /// The level of skill `skill`, given as its id or its rathena name such as `"MC_LOUD"`. `0` when not learned.
    pub fn skill_level(&self, skill: impl Into<crate::Val>) -> Result<i32, Stop> {
        self.ctx.call(script_sdk::Function::GetSkillLv, crate::args![skill.into()])?.number()
    }

    /// Adds base and job experience to the attached player. Errors when either amount is negative.
    pub fn give_experience(&self, base: i32, job: i32) -> Script {
        self.ctx.call(script_sdk::Function::GetExperience, crate::args![base, job]).map(|_| ())
    }
}

impl Player<'_, '_> {
    /// Whether `target` (the attached player when `None`) has a cart.
    pub fn has_cart(&self, target: Option<i32>) -> Result<bool, Stop> {
        self.check(script_sdk::Function::CheckCart, target)
    }

    pub fn has_falcon(&self, target: Option<i32>) -> Result<bool, Stop> {
        self.check(script_sdk::Function::CheckFalcon, target)
    }

    pub fn is_riding(&self, target: Option<i32>) -> Result<bool, Stop> {
        self.check(script_sdk::Function::CheckRiding, target)
    }

    pub fn is_mounting(&self, target: Option<i32>) -> Result<bool, Stop> {
        self.check(script_sdk::Function::IsMounting, target)
    }

    fn check(&self, function: script_sdk::Function, target: Option<i32>) -> Result<bool, Stop> {
        let mut arguments = crate::args![];
        arguments.extend(target.map(crate::Val::from));
        Ok(self.ctx.call(function, arguments)?.number()? != 0)
    }

    /// A stat by its rathena name, such as `"bStr"`, as rathena's `readparam`.
    pub fn read_param(&self, name: &str) -> Result<i32, Stop> {
        self.ctx.call(script_sdk::Function::ReadParam, crate::args![name])?.number()
    }

    /// Changes the attached player to job `job`, such as `constants::JOB_KNIGHT`.
    pub fn change_job(&self, job: i32) -> Script {
        self.ctx.call(script_sdk::Function::JobChange, crate::args![job]).map(|_| ())
    }

    /// Ends status `kind`, such as `constants::SC_BLESSING`, on `target` (the attached player when `None`).
    /// Pass `-1` as `kind` to end every status.
    pub fn end_status(&self, kind: impl Into<crate::Val>, target: Option<i32>) -> Script {
        let mut arguments = crate::args![kind.into()];
        arguments.extend(target.map(crate::Val::from));
        self.ctx.call(script_sdk::Function::EndStatus, arguments).map(|_| ())
    }
}

impl Player<'_, '_> {
    /// Sets one of the attached player's looks, such as hair style, as rathena's `setlook`.
    pub fn set_look(&self, look_type: i32, value: i32) -> Script {
        self.ctx.call(script_sdk::Function::SetLook, crate::args![look_type, value]).map(|_| ())
    }

    /// The item id worn in equipment slot `slot`, such as `constants::EQI_HEAD_TOP`, or `0` when empty.
    pub fn equipped_item_id(&self, slot: &str) -> Result<i32, Stop> {
        self.ctx.call(script_sdk::Function::GetEquipId, crate::args![slot])?.number()
    }

    /// The refine level of the item in equipment slot `slot`.
    pub fn equipped_refine(&self, slot: &str) -> Result<i32, Stop> {
        self.ctx.call(script_sdk::Function::GetEquipRefineryCnt, crate::args![slot])?.number()
    }

    /// The partner id of `target` (the attached player when `None`), or `0` without a partner.
    pub fn partner_id(&self, target: Option<i32>) -> Result<i32, Stop> {
        let mut arguments = crate::args![];
        arguments.extend(target.map(crate::Val::from));
        self.ctx.call(script_sdk::Function::GetPartnerId, arguments)?.number()
    }

    /// Whether the attached player wears an item in `slot`, such as `constants::EQI_HAND_R`.
    pub fn equipped_in(&self, slot: &str) -> Result<bool, Stop> {
        Ok(self.ctx.call(script_sdk::Function::GetEquipIsEquipped, crate::args![slot])?.number()? != 0)
    }

    /// The card in `card` (0 to 3) of the item in `slot`: `0` when the slot is empty, `-1` when nothing is worn there.
    pub fn equipped_card(&self, slot: &str, card: i32) -> Result<i32, Stop> {
        self.ctx.call(script_sdk::Function::GetEquipCardId, crate::args![slot, card])?.number()
    }

    /// Sets the save point of the attached player, or of `char_id`. `range` adds a random offset of up to that many
    /// cells around `x` and `y`.
    pub fn save_point(&self, map: &str, x: i32, y: i32, range: Option<(i32, i32)>, char_id: Option<i32>) -> Script {
        let mut arguments = crate::args![map, x, y];
        if let Some((range_x, range_y)) = range {
            arguments.extend(crate::args![range_x, range_y]);
        }
        arguments.extend(char_id.map(crate::Val::from));
        self.ctx.call(script_sdk::Function::SavePoint, arguments).map(|_| ())
    }

    /// Gives the attached player skill `skill`, an id or a name such as `"NV_BASIC"`, at `level`.
    pub fn grant_skill(&self, skill: impl Into<crate::Val>, level: i32, grant: SkillGrant) -> Script {
        self.ctx.call(script_sdk::Function::Skill, crate::args![skill.into(), level, grant.flag()]).map(|_| ())
    }

    /// The map name and position of the attached player, as rathena's `getmapxy`.
    pub fn map_xy(&self) -> Result<(String, i32, i32), Stop> {
        let value = self.ctx.call(script_sdk::Function::GetMapXy, crate::args![crate::constants::BL_PC])?;
        let parts = value.into_array().unwrap_or_default();
        let [map, x, y] = parts.as_slice() else {
            return Err(Stop::Error("getmapxy returned no position".into()));
        };
        Ok((map.text(), x.number()?, y.number()?))
    }

    /// Hires mercenary `class` for the attached player for `milliseconds`, as rathena's `mercenary_create`. The class
    /// must be in the mercenary database, and the player can hold only one contract at a time.
    pub fn hire_mercenary(&self, class: i32, milliseconds: i32) -> Script {
        self.ctx.call(script_sdk::Function::MercenaryCreate, crate::args![class, milliseconds]).map(|_| ())
    }
}

impl Player<'_, '_> {
    /// One of the attached player's looks, such as `constants::LOOK_HAIR`, as rathena's `getlook`.
    pub fn look(&self, look_type: i32) -> Result<i32, Stop> {
        self.ctx.call(script_sdk::Function::GetLook, crate::args![look_type])?.number()
    }

    /// Shows `text` in the attached player's chat window, as rathena's `message`.
    pub fn message(&self, text: &str) -> Script {
        self.ctx.call(script_sdk::Function::Message, crate::args![text]).map(|_| ())
    }

    /// Restores `hp` and `sp` percent of the attached player's maximum. Negative values take them away.
    pub fn percent_heal(&self, hp: i32, sp: i32) -> Script {
        self.ctx.call(script_sdk::Function::PercentHeal, crate::args![hp, sp]).map(|_| ())
    }

    /// Opens the attached player's storage.
    pub fn open_storage(&self) -> Script {
        self.ctx.call(script_sdk::Function::OpenStorage, crate::args![]).map(|_| ())
    }

    /// Resets the attached player's levels, as rathena's `resetlvl`. Kind `1` is the reset of a rebirth.
    pub fn reset_level(&self, kind: i32) -> Script {
        self.ctx.call(script_sdk::Function::ResetLevel, crate::args![kind]).map(|_| ())
    }
}

impl Player<'_, '_> {
    /// Gives the attached player a cart, or takes it away.
    pub fn set_cart(&self, on: bool) -> Script {
        self.ctx.call(script_sdk::Function::SetCart, crate::args![on]).map(|_| ())
    }

    pub fn set_falcon(&self, on: bool) -> Script {
        self.ctx.call(script_sdk::Function::SetFalcon, crate::args![on]).map(|_| ())
    }

    pub fn set_riding(&self, on: bool) -> Script {
        self.ctx.call(script_sdk::Function::SetRiding, crate::args![on]).map(|_| ())
    }

    /// Marries the attached player to the online player named `partner`. Returns whether the marriage happened.
    pub fn marry(&self, partner: &str) -> Result<bool, Stop> {
        Ok(self.ctx.call(script_sdk::Function::Marriage, crate::args![partner])?.number()? == 1)
    }

    /// Ends the attached player's marriage.
    pub fn divorce(&self) -> Script {
        self.ctx.call(script_sdk::Function::Divorce, crate::args![]).map(|_| ())
    }
}

/// How a granted skill is kept, as rathena's `skill` flag.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkillGrant {
    /// Saved with the character.
    Permanent,
    /// Lost after a while, as rathena's default.
    Temporary,
    /// Adds the level to the skill's current level, as a stackable bonus.
    Stacked,
    /// Saved, but kept when the skill tree is reset or the job changes.
    KeptOnReset,
}

impl SkillGrant {
    fn flag(self) -> i32 {
        match self {
            SkillGrant::Permanent => 0,
            SkillGrant::Temporary => 1,
            SkillGrant::Stacked => 2,
            SkillGrant::KeptOnReset => 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use script_sdk::{Function, Request, Value};

    use crate::player::SkillGrant;
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

    #[test]
    fn id_queries_send_their_kind() {
        let transport = MockTransport::new(|_| Ok(Value::Number(12)));
        let ctx = Ctx::new(&transport);
        assert_eq!(ctx.player().guild_id(), Ok(12));
        assert_eq!(transport.calls(Function::GetCharacterId)[0], vec![Value::new_number(2)]);
    }

    #[test]
    fn give_experience_sends_base_then_job() {
        let transport = MockTransport::silent();
        Ctx::new(&transport).player().give_experience(100, 20).unwrap();
        assert_eq!(transport.calls(Function::GetExperience)[0], vec![Value::new_number(100), Value::new_number(20)]);
    }

    #[test]
    fn checks_send_no_target_unless_given() {
        let transport = MockTransport::new(|_| Ok(Value::Number(1)));
        let ctx = Ctx::new(&transport);
        assert_eq!(ctx.player().is_riding(None), Ok(true));
        assert_eq!(ctx.player().has_cart(Some(9)), Ok(true));
        assert_eq!(transport.calls(Function::CheckRiding)[0].len(), 0);
        assert_eq!(transport.calls(Function::CheckCart)[0], vec![Value::new_number(9)]);
    }

    #[test]
    fn end_status_sends_the_kind_first() {
        let transport = MockTransport::silent();
        Ctx::new(&transport).player().end_status(-1, None).unwrap();
        assert_eq!(transport.calls(Function::EndStatus)[0], vec![Value::new_number(-1)]);
    }

    #[test]
    fn equipped_item_id_sends_the_slot_name() {
        let transport = MockTransport::new(|_| Ok(Value::Number(2101)));
        let ctx = Ctx::new(&transport);
        assert_eq!(ctx.player().equipped_item_id("EQI_HEAD_TOP"), Ok(2101));
        assert_eq!(transport.calls(Function::GetEquipId)[0], vec![Value::new_string("EQI_HEAD_TOP".into())]);
    }

    #[test]
    fn hire_mercenary_sends_the_class_then_the_duration() {
        let transport = MockTransport::silent();
        Ctx::new(&transport).player().hire_mercenary(6017, 60_000).unwrap();
        assert_eq!(transport.calls(Function::MercenaryCreate)[0], vec![Value::new_number(6017), Value::new_number(60_000)]);
    }

    #[test]
    fn grant_skill_sends_the_rathena_flag() {
        let transport = MockTransport::silent();
        Ctx::new(&transport).player().grant_skill("NV_BASIC", 9, SkillGrant::KeptOnReset).unwrap();
        assert_eq!(transport.calls(Function::Skill)[0], vec![Value::new_string("NV_BASIC".into()), Value::new_number(9), Value::new_number(3)]);
    }

    #[test]
    fn marry_succeeds_only_on_a_reply_of_one() {
        let transport = MockTransport::new(|_| Ok(Value::Number(1)));
        assert_eq!(Ctx::new(&transport).player().marry("Partner"), Ok(true));
        let transport = MockTransport::new(|_| Ok(Value::Number(0)));
        assert_eq!(Ctx::new(&transport).player().marry("Partner"), Ok(false));
        assert_eq!(transport.calls(Function::Marriage)[0], vec![Value::new_string("Partner".into())]);
    }

    #[test]
    fn mounts_send_the_flag_as_a_number() {
        let transport = MockTransport::silent();
        Ctx::new(&transport).player().set_falcon(false).unwrap();
        assert_eq!(transport.calls(Function::SetFalcon)[0], vec![Value::new_number(0)]);
    }

    #[test]
    fn save_point_fills_the_range_before_the_character() {
        let transport = MockTransport::silent();
        let ctx = Ctx::new(&transport);
        ctx.player().save_point("prontera", 150, 150, None, Some(7)).unwrap();
        ctx.player().save_point("prontera", 150, 150, Some((2, 2)), None).unwrap();
        let calls = transport.calls(Function::SavePoint);
        assert_eq!(calls[0].len(), 4);
        assert_eq!(calls[0][3], Value::new_number(7));
        assert_eq!(calls[1].len(), 5);
    }
}
