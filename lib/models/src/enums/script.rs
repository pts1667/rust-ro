use super::{EnumWithMaskValueU32, WithMaskValueU32};

#[derive(Debug, Copy, Clone, WithMaskValueU32)]
pub enum BroadcastFlag {
    #[mask_value = 1]
    Map,
    Area,
    ReservedTarget,
    NpcSource,
    Blue,
    WarOfEmperium,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum ScriptSendTarget {
    All = 0,
    Map = 1,
    Area = 2,
    AreaWithoutSelf = 3,
    SelfOnly = 24,
}
