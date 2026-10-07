//! `db/pre-re/item_noequip.txt`: items that cannot be used on versus maps or in restricted zones.
//! Every pre-renewal entry is a consumable, so nothing needs to be unequipped on map change.

use models::enums::map::MapRestrictionZone;
use models::enums::EnumWithMaskValueU32;

use crate::server::model::map_flags::{MapFlag, MapFlags};

#[derive(Clone, Copy)]
enum Restriction {
    Pvp,
    Gvg,
    Battleground,
    Zone(MapRestrictionZone),
}

use Restriction::{Battleground, Gvg, Pvp, Zone};
use MapRestrictionZone::{Zone1, Zone2, Zone3, Zone7};

const NO_EQUIP: &[(u32, Restriction)] = &[
    (14529, Pvp),
    (14529, Gvg),
    (14529, Zone(Zone7)),
    (12218, Gvg),
    (12218, Battleground),
    (14590, Gvg),
    (14590, Battleground),
    (601, Zone(Zone1)),
    (601, Zone(Zone2)),
    (601, Zone(Zone3)),
    (605, Zone(Zone1)),
    (506, Zone(Zone1)),
    (525, Zone(Zone1)),
    (602, Zone(Zone2)),
    (12212, Zone(Zone2)),
    (14582, Zone(Zone2)),
    (14583, Zone(Zone2)),
    (14584, Zone(Zone2)),
    (14585, Zone(Zone2)),
];

/// `itemdb_isNoEquip`
pub fn is_no_equip(item_id: u32, flags: &MapFlags) -> bool {
    NO_EQUIP.iter().filter(|(id, _)| *id == item_id).any(|(_, restriction)| match restriction {
        Pvp => flags.enabled(MapFlag::Pvp),
        Gvg => flags.is_gvg(),
        Battleground => flags.enabled(MapFlag::Battleground),
        Zone(zone) => flags.get(MapFlag::Restricted, None) as u32 & zone.as_flag() != 0,
    })
}
