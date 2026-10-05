use crate::enums::*;

#[derive(WithMaskValueU16, Debug, Copy, Clone, PartialEq, Eq)]
pub enum CellType {
    Walkable,
    Shootable,
    Water,
    Boot,
    Basilica,
    LandProtector,
    NoVending,
    NoChat,
    Icewall,
    NoIceWall,
    NoSkill,
    Warp,
    Mob,
}
