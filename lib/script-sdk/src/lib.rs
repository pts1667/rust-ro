use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

pub const ABI_VERSION: u32 = 1;
pub const MAX_MESSAGE_BYTES: usize = 65_536;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Value {
    Number(i32),
    String(String),
    Array(Vec<Value>),
    Null,
}

impl Default for Value {
    fn default() -> Self {
        Self::Number(0)
    }
}

impl Value {
    pub fn new_number(value: i32) -> Self {
        Self::Number(value)
    }

    pub fn new_string(value: String) -> Self {
        Self::String(value)
    }

    pub fn number_value(&self) -> Result<i32, String> {
        match self {
            Self::Number(value) => Ok(*value),
            _ => Err("Expected a number".into()),
        }
    }

    pub fn string_value(&self) -> Result<&String, String> {
        match self {
            Self::String(value) => Ok(value),
            _ => Err("Expected a string".into()),
        }
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Self::Number(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn truthy(&self) -> bool {
        match self {
            Self::Number(n) => *n != 0,
            Self::String(s) => !s.is_empty(),
            Self::Array(v) => !v.is_empty(),
            Self::Null => false,
        }
    }

    pub fn text(&self) -> String {
        match self {
            Self::Number(n) => n.to_string(),
            Self::String(s) => s.clone(),
            Self::Array(_) => "[array]".into(),
            Self::Null => String::new(),
        }
    }

    pub fn add(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::Number(a), Self::Number(b)) => Self::Number(a.wrapping_add(b)),
            (a, b) => Self::String(a.text() + &b.text()),
        }
    }

    pub fn binary(self, operation: &str, rhs: Self) -> Result<Self, String> {
        if operation == "+" {
            return Ok(self.add(rhs));
        }
        if operation == "==" || operation == "!=" {
            let equal = self == rhs;
            return Ok(Self::Number(i32::from(if operation == "==" { equal } else { !equal })));
        }
        let a = self.number_value()?;
        let b = rhs.number_value()?;
        let value = match operation {
            "-" => a.wrapping_sub(b),
            "*" => a.wrapping_mul(b),
            "/" => a.checked_div(b).ok_or("Invalid division")?,
            "%" => a.checked_rem(b).ok_or("Invalid remainder")?,
            "&" => a & b,
            "|" => a | b,
            "^" => a ^ b,
            "<<" => a.wrapping_shl(b as u32),
            ">>" => a.wrapping_shr(b as u32),
            "<" => i32::from(a < b),
            ">" => i32::from(a > b),
            "<=" => i32::from(a <= b),
            ">=" => i32::from(a >= b),
            _ => return Err(format!("Unknown operator {operation}")),
        };
        Ok(Self::Number(value))
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Self::Number(value)
    }
}
impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self::String(value.into())
    }
}
impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}
impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.text())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VariableScope {
    Character,
    Account,
    Server,
    ServerTemporary,
    CharacterTemporary,
    Npc,
    NpcInstance,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Variable {
    pub scope: VariableScope,
    pub name: String,
    pub index: u32,
    pub value: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Function {
    Print,
    Mes,
    Close,
    Next,
    Select,
    InputNumber,
    InputString,
    Message,
    DispBottom,
    Cutin,
    GetLook,
    SetLook,
    GetBattleFlag,
    StrCharInfo,
    Warp,
    JobName,
    EaClass,
    RoClass,
    JobChange,
    CheckFalcon,
    CheckCart,
    CheckRiding,
    IsMounting,
    CheckMadogear,
    SetCart,
    SetFalcon,
    SetRiding,
    Bonus,
    Bonus2,
    Bonus3,
    Bonus4,
    Bonus5,
    Skill,
    ItemSkill,
    ReadParam,
    GetRefine,
    GetSkillLv,
    IsEquipped,
    GetEquipId,
    GetEquipRefineryCnt,
    GetItemInfo,
    Rand,
    Max,
    Min,
    Pow,
    GetTime,
    VipStatus,
    GetPartnerId,
    ItemHeal,
    PercentHeal,
    Heal,
    GetItem,
    DelItem,
    CountItem,
    SpecialEffect,
    SkillEffect,
    StartStatus,
    StartStatus2,
    StartStatus4,
    EndStatus,
    AutoBonus,
    AutoBonus2,
    AutoBonus3,
    RandomGroupItem,
    GetGroupItem,
    Monster,
    Produce,
    Pet,
    BirthPet,
    GuildExperience,
    Cooking,
    CallFunction,
    MercenaryCreate,
    MercenaryStartStatus,
    MercenaryHeal,
    SetFont,
    SearchStores,
    Homevolution,
    Announce,
    BuyingStore,
    GetExperience,
    Shop,
    ResetLevel,
    ResetSkills,
    OpenStorage,
    PartyWarp,
    UnitSkill,
    GetFame,
    GetFameRank,
    AddFame,
    GuildOpenStorage,
    GetPetInfo,
    PetSkillBonus,
    PetRecovery,
    PetSkillAttack,
    PetSkillAttack2,
    PetSkillSupport,
    PetLoot,
    GetMapFlag,
    SetMapFlag,
    RemoveMapFlag,
    PvpOn,
    PvpOff,
    GvgOn,
    GvgOff,
    GetCharacterId,
    UnitSkillToId,
    UnitSkillToPosition,
    GetUnitData,
    SetUnitData,
    GetNpcId,
    DoEvent,
    DoNpcEvent,
    PetAutoBonus,
    PetAutoBonus2,
    PetAutoBonus3,
    AddTimer,
    DeleteTimer,
    AddTimerCount,
    InitNpcTimer,
    StartNpcTimer,
    StopNpcTimer,
    SetNpcTimer,
    GetNpcTimer,
    AttachNpcTimer,
    DetachNpcTimer,
    SetMapFlagNoSave,
    SavePoint,
    GetSavePoint,
    WarpPortal,
    AgitStart,
    AgitEnd,
    AgitCheck,
    GetCastleData,
    SetCastleData,
    Marriage,
    Divorce,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Request {
    Constant(String),
    Read(String),
    Write { name: String, value: Value },
    VariableRead { scope: VariableScope, name: String, index: u32 },
    VariablesWrite(Vec<Variable>),
    VariablesIncrement(Vec<Variable>),
    Arguments,
    Inventory,
    Purchase(Vec<(u32, i16, i32)>),
    Sale(Vec<(usize, i16, i32)>),
    Call { function: Function, arguments: Vec<Value> },
    ReportError(String),
}

pub type Reply = Result<Value, String>;

pub struct Context;

impl Context {
    pub fn request(&self, request: Request) -> Reply {
        #[cfg(target_arch = "wasm32")]
        {
            #[link(wasm_import_module = "rust_ro")]
            extern "C" {
                fn invoke(request: *const u8, length: usize, response: *mut u8, capacity: usize) -> i32;
            }
            let bytes = serde_json::to_vec(&request).map_err(|e| e.to_string())?;
            if bytes.len() > MAX_MESSAGE_BYTES {
                return Err("Request too large".into());
            }
            let mut response = vec![0u8; MAX_MESSAGE_BYTES];
            let length = unsafe { invoke(bytes.as_ptr(), bytes.len(), response.as_mut_ptr(), response.len()) };
            if length < 0 || length as usize > response.len() {
                return Err("Invalid host response".into());
            }
            serde_json::from_slice(&response[..length as usize]).map_err(|e| e.to_string())?
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = request;
            Err("Script context requires a WebAssembly host".into())
        }
    }

    pub fn constant(&self, name: &str) -> Reply {
        self.request(Request::Constant(name.into()))
    }

    pub fn read(&self, name: &str) -> Reply {
        self.request(Request::Read(name.into()))
    }

    pub fn write(&self, name: &str, value: Value) -> Result<(), String> {
        self.request(Request::Write { name: name.into(), value }).map(|_| ())
    }

    pub fn call(&self, function: Function, arguments: Vec<Value>) -> Reply {
        self.request(Request::Call { function, arguments })
    }

    pub fn get_unit_data(&self, actor_id: u32) -> Reply {
        self.call(Function::GetUnitData, vec![Value::Number(actor_id as i32)])
    }

    pub fn set_unit_data(&self, actor_id: u32, field: i32, value: Value) -> Reply {
        self.call(Function::SetUnitData, vec![
            Value::Number(actor_id as i32),
            Value::Number(field),
            value,
        ])
    }

    pub fn npc_id(&self) -> Result<u32, String> {
        self.call(Function::GetNpcId, vec![0.into()])?.number_value().map(|id| id as u32)
    }

    pub fn npc_event(&self, label: &str) -> Result<(), String> {
        self.call(Function::DoNpcEvent, vec![label.into()]).map(|_| ())
    }

    pub fn player_event(&self, label: &str) -> Result<(), String> {
        self.call(Function::DoEvent, vec![label.into()]).map(|_| ())
    }

    pub fn get_map_flag(&self, map: &str, flag: i32, parameter: Option<i32>) -> Result<i32, String> {
        let mut arguments = vec![map.into(), flag.into()];
        arguments.extend(parameter.map(Value::from));
        self.call(Function::GetMapFlag, arguments)?.number_value()
    }

    pub fn set_map_flag(&self, map: &str, flag: i32, parameters: &[i32]) -> Result<(), String> {
        let mut arguments = vec![map.into(), flag.into()];
        arguments.extend(parameters.iter().copied().map(Value::from));
        self.call(Function::SetMapFlag, arguments).map(|_| ())
    }

    pub fn remove_map_flag(&self, map: &str, flag: i32) -> Result<(), String> {
        self.call(Function::RemoveMapFlag, vec![map.into(), flag.into()]).map(|_| ())
    }

    pub fn set_map_no_save(&self, map: &str, alternate_map: &str, x: i32, y: i32) -> Result<(), String> {
        self.call(Function::SetMapFlagNoSave, vec![
            map.into(),
            alternate_map.into(),
            x.into(),
            y.into(),
        ])
        .map(|_| ())
    }

    pub fn save_point(&self, map: &str, x: u16, y: u16) -> Result<(), String> {
        self.call(Function::SavePoint, vec![map.into(), i32::from(x).into(), i32::from(y).into()])
            .map(|_| ())
    }

    pub fn get_save_point(&self, kind: i32, char_id: Option<u32>) -> Reply {
        let mut arguments = vec![kind.into()];
        if let Some(id) = char_id {
            arguments.push(i32::try_from(id).map_err(|_| "Character ID is out of range")?.into());
        }
        self.call(Function::GetSavePoint, arguments)
    }

    pub fn warp_portal(&self, x: u16, y: u16, map: &str, destination_x: u16, destination_y: u16) -> Result<(), String> {
        self.call(Function::WarpPortal, vec![i32::from(x).into(), i32::from(y).into(), map.into(),
            i32::from(destination_x).into(), i32::from(destination_y).into()]).map(|_| ())
    }

    pub fn set_pvp(&self, map: &str, enabled: bool) -> Result<(), String> {
        self.call(if enabled { Function::PvpOn } else { Function::PvpOff }, vec![map.into()])
            .map(|_| ())
    }

    pub fn set_gvg(&self, map: &str, enabled: bool) -> Result<(), String> {
        self.call(if enabled { Function::GvgOn } else { Function::GvgOff }, vec![map.into()])
            .map(|_| ())
    }

    pub fn mes(&self, text: impl Into<String>) -> Result<(), String> {
        self.call(Function::Mes, vec![Value::String(text.into())]).map(|_| ())
    }

    pub fn next(&self) -> Result<(), String> {
        self.call(Function::Next, vec![]).map(|_| ())
    }

    pub fn select(&self, options: &[String]) -> Result<usize, String> {
        let selected = self
            .call(Function::Select, options.iter().map(|s| Value::String(s.clone())).collect())?
            .number_value()?;
        if selected < 1 || selected as usize > options.len() {
            return Err("Conversation cancelled".into());
        }
        Ok(selected as usize - 1)
    }

    pub fn close(&self) -> Result<(), String> {
        self.call(Function::Close, vec![]).map(|_| ())
    }
}
