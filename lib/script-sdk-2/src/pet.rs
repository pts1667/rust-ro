use script_sdk::Function;

use crate::args;
use crate::ctx::Ctx;
use crate::flow::{Script, Stop};
use crate::value::Val;

impl<'a> Ctx<'a> {
    /// Pets and taming, such as `ctx.pet().info(PetInfo::Name, None)`.
    pub fn pet<'c>(&'c self) -> Pets<'c, 'a> {
        Pets { ctx: self }
    }
}

pub struct Pets<'c, 'a> {
    ctx: &'c Ctx<'a>,
}

/// A field of the active pet, as rathena's `PETINFO_*` constants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PetInfo {
    Id,
    Class,
    /// `"null"` when there is no pet.
    Name,
    Intimacy,
    Hunger,
    Renamed,
    Level,
    BlockId,
    EggId,
    FoodId,
}

impl PetInfo {
    fn field(self) -> i32 {
        match self {
            PetInfo::Id => 0,
            PetInfo::Class => 1,
            PetInfo::Name => 2,
            PetInfo::Intimacy => 3,
            PetInfo::Hunger => 4,
            PetInfo::Renamed => 5,
            PetInfo::Level => 6,
            PetInfo::BlockId => 7,
            PetInfo::EggId => 8,
            PetInfo::FoodId => 9,
        }
    }
}

/// Which monsters a taming item can catch, as rathena's `PET_CATCH_*` flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PetCatch {
    /// Only monsters in the pet database that match the lure.
    #[default]
    Normal,
    /// Any monster except bosses, whatever its immunity.
    UniversalNoBoss,
    /// Any monster, bosses included, whatever its immunity.
    UniversalAll,
}

impl PetCatch {
    fn flag(self) -> i32 {
        match self {
            PetCatch::Normal => 0,
            PetCatch::UniversalNoBoss => 1,
            PetCatch::UniversalAll => 2,
        }
    }
}

impl Pets<'_, '_> {
    /// A field of the active pet of the attached player, or of `char_id` when given. Numeric fields are `0` when there
    /// is no pet, and `info(PetInfo::Name, ..)` is `"null"`.
    pub fn info(&self, field: PetInfo, char_id: Option<i32>) -> Result<Val, Stop> {
        let mut arguments = args![field.field()];
        arguments.extend(char_id.map(Val::from));
        self.ctx.call(Function::GetPetInfo, arguments)
    }

    /// Starts the taming cursor for `lure`, an item id or a pet class that has a lure, as rathena's `pet`.
    pub fn catch(&self, lure: i32, mode: PetCatch) -> Script {
        self.ctx.call(Function::Pet, args![lure, mode.flag()]).map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use script_sdk::{Function, Value};

    use crate::pet::{PetCatch, PetInfo};
    use crate::transport::MockTransport;
    use crate::Ctx;

    #[test]
    fn info_sends_the_field_and_the_optional_character() {
        let transport = MockTransport::new(|_| Ok(Value::Number(0)));
        let ctx = Ctx::new(&transport);
        ctx.pet().info(PetInfo::Level, None).unwrap();
        ctx.pet().info(PetInfo::Name, Some(150000)).unwrap();
        let calls = transport.calls(Function::GetPetInfo);
        assert_eq!(calls[0], vec![Value::new_number(6)]);
        assert_eq!(calls[1], vec![Value::new_number(2), Value::new_number(150000)]);
    }

    #[test]
    fn catch_sends_the_lure_then_the_flag() {
        let transport = MockTransport::silent();
        Ctx::new(&transport).pet().catch(5001, PetCatch::UniversalAll).unwrap();
        assert_eq!(transport.calls(Function::Pet)[0], vec![Value::new_number(5001), Value::new_number(2)]);
    }
}
