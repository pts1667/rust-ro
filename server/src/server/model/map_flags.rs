use std::collections::BTreeMap;

use models::enums::map::{MapActorType, MapRestrictionZone, NightmareDropLocation};
use models::enums::{EnumWithMaskValueU8, EnumWithMaskValueU16, EnumWithMaskValueU32};
use serde::Deserialize;

macro_rules! map_flags {
    ($($variant:ident = $id:literal => $name:literal),+ $(,)?) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
        #[repr(u16)]
        pub enum MapFlag { $($variant = $id),+ }
        impl MapFlag { pub const CATALOGUE: &'static [(Self, &'static str)] = &[$((Self::$variant,$name)),+]; }
    };
}
map_flags! {
    NoMemo=0=>"nomemo",NoTeleport=1=>"noteleport",NoSave=2=>"nosave",NoBranch=3=>"nobranch",
    NoPenalty=4=>"nopenalty",NoZenyPenalty=5=>"nozenypenalty",Pvp=6=>"pvp",PvpNoParty=7=>"pvp_noparty",
    PvpNoGuild=8=>"pvp_noguild",Gvg=9=>"gvg",GvgNoParty=10=>"gvg_noparty",NoTrade=11=>"notrade",
    NoSkill=12=>"noskill",NoWarp=13=>"nowarp",PartyLock=14=>"partylock",NoIceWall=15=>"noicewall",
    Snow=16=>"snow",Fog=17=>"fog",Sakura=18=>"sakura",Leaves=19=>"leaves",NoGo=22=>"nogo",
    Clouds=23=>"clouds",Clouds2=24=>"clouds2",Fireworks=25=>"fireworks",GvgCastle=26=>"gvg_castle",
    GvgDungeon=27=>"gvg_dungeon",NightEnabled=28=>"nightenabled",NoBaseExp=29=>"nobaseexp",NoJobExp=30=>"nojobexp",
    NoMobLoot=31=>"nomobloot",NoMvpLoot=32=>"nomvploot",NoReturn=33=>"noreturn",NoWarpTo=34=>"nowarpto",
    PvpNightmareDrop=35=>"pvp_nightmaredrop",Restricted=36=>"restricted",NoCommand=37=>"nocommand",NoDrop=38=>"nodrop",
    Jexp=39=>"jexp",Bexp=40=>"bexp",NoVending=41=>"novending",LoadEvent=42=>"loadevent",NoChat=43=>"nochat",
    NoExpPenalty=44=>"noexppenalty",GuildLock=45=>"guildlock",Town=46=>"town",AutoTrade=47=>"autotrade",
    AllowKs=48=>"allowks",MonsterNoTeleport=49=>"monster_noteleport",PvpNoCalcRank=50=>"pvp_nocalcrank",
    Battleground=51=>"battleground",Reset=52=>"reset",NoMapChannelAutoJoin=53=>"nomapchannelautojoin",
    NoUseCart=54=>"nousecart",NoItemConsumption=55=>"noitemconsumption",NoSunMoonStarMiracle=56=>"nosunmoonstarmiracle",
    ForceMinEffect=57=>"forcemineffect",NoLockOn=58=>"nolockon",NoTomb=59=>"notomb",SkillDamage=60=>"skill_damage",
    NoCostume=61=>"nocostume",HideMobHpBar=64=>"hidemobhpbar",NoLoot=65=>"noloot",NoExp=66=>"noexp",SkillDuration=69=>"skill_duration",
    NoPetCapture=74=>"nopetcapture",NoBuyingStore=75=>"nobuyingstore",NoDynamicNpc=76=>"nodynamicnpc",NoBank=77=>"nobank",
    SpecialPopup=78=>"specialpopup",InvincibleTime=80=>"invincible_time",
}
impl MapFlag {
    pub fn from_id(id: i32) -> Result<Self, String> {
        Self::CATALOGUE
            .iter()
            .find(|(flag, _)| *flag as i32 == id)
            .map(|(flag, _)| *flag)
            .ok_or("Unsupported pre-renewal map flag".into())
    }

    pub fn from_name(name: &str) -> Result<Self, String> {
        let name = name.trim().to_ascii_lowercase();
        let name = name.strip_prefix("mf_").unwrap_or(&name);
        Self::CATALOGUE
            .iter()
            .find(|(_, value)| *value == name)
            .map(|(flag, _)| *flag)
            .ok_or_else(|| format!("Unknown map flag {name}"))
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapFlags {
    values: BTreeMap<MapFlag, i32>,
    skill_damage: [i32; 4],
    skill_damage_caster: u16,
    specific_skill_damage: BTreeMap<u32, (u16, [i32; 4])>,
    skill_duration: BTreeMap<u32, u16>,
    pub save: Option<(String, u16, u16)>,
    pub nightmare_drops: Vec<(i32, u8, u16)>,
}
impl Default for MapFlags {
    fn default() -> Self {
        Self {
            values: BTreeMap::from([(MapFlag::Bexp, 100), (MapFlag::Jexp, 100)]),
            skill_damage: [0; 4],
            skill_damage_caster: 0,
            specific_skill_damage: BTreeMap::new(),
            skill_duration: BTreeMap::new(),
            save: None,
            nightmare_drops: vec![],
        }
    }
}
impl MapFlags {
    pub fn enabled(&self, flag: MapFlag) -> bool {
        self.get(flag, None) != 0
    }

    pub fn is_gvg(&self) -> bool {
        [MapFlag::Gvg, MapFlag::GvgCastle, MapFlag::GvgDungeon]
            .into_iter()
            .any(|flag| self.enabled(flag))
    }

    pub fn versus(&self, siege_active: bool) -> bool {
        self.enabled(MapFlag::Pvp)
            || self.enabled(MapFlag::Gvg)
            || self.enabled(MapFlag::GvgDungeon)
            || (siege_active && self.enabled(MapFlag::GvgCastle))
            || self.enabled(MapFlag::Battleground)
    }

    pub fn get(&self, flag: MapFlag, argument: Option<i32>) -> i32 {
        let pair = match flag {
            MapFlag::NoExp => Some((MapFlag::NoBaseExp, MapFlag::NoJobExp)),
            MapFlag::NoLoot => Some((MapFlag::NoMobLoot, MapFlag::NoMvpLoot)),
            MapFlag::NoPenalty => Some((MapFlag::NoExpPenalty, MapFlag::NoZenyPenalty)),
            _ => None,
        };
        if let Some((a, b)) = pair {
            return i32::from(self.enabled(a) && self.enabled(b));
        }
        if flag == MapFlag::SkillDamage {
            if let Some(index @ 0..=3) = argument {
                return self.skill_damage[index as usize];
            }
            if argument == Some(5) {
                return i32::from(self.skill_damage_caster);
            }
        }
        if flag == MapFlag::SkillDuration {
            if let Some(id) = argument {
                return i32::from(self.skill_duration.get(&(id as u32)).copied().unwrap_or(100));
            }
        }
        self.values.get(&flag).copied().unwrap_or(0)
    }

    pub fn set(&mut self, flag: MapFlag, enabled: bool, arguments: &[i32]) -> Result<(), String> {
        let pair = match flag {
            MapFlag::NoExp => Some((MapFlag::NoBaseExp, MapFlag::NoJobExp)),
            MapFlag::NoLoot => Some((MapFlag::NoMobLoot, MapFlag::NoMvpLoot)),
            MapFlag::NoPenalty => Some((MapFlag::NoExpPenalty, MapFlag::NoZenyPenalty)),
            _ => None,
        };
        if let Some((a, b)) = pair {
            self.set(a, enabled, &[])?;
            self.set(b, enabled, &[])?;
            return Ok(());
        }
        if flag == MapFlag::Restricted {
            let mut mask = self.get(flag, None) as u32;
            if !enabled && arguments.is_empty() {
                mask = 0;
            } else {
                let zone = MapRestrictionZone::from_zone(arguments.first().copied().unwrap_or(0)).ok_or("Invalid map restriction zone")?;
                if enabled {
                    mask |= zone.as_flag();
                } else {
                    mask &= !zone.as_flag();
                }
            }
            self.values.insert(flag, mask as i32);
            return Ok(());
        }
        if flag == MapFlag::SkillDamage {
            if enabled {
                match arguments {
                    [] => {
                        self.skill_damage_caster |= MapActorType::All.as_flag();
                        self.skill_damage = [0; 4];
                    }
                    [rate, kind] => {
                        let index = usize::try_from(*kind)
                            .ok()
                            .filter(|index| *index < 4)
                            .ok_or("Invalid skill damage target kind")?;
                        self.skill_damage[index] = (*rate).clamp(-100, 100000);
                        self.skill_damage_caster |= MapActorType::All.as_flag();
                    }
                    [skill, caster, pc, mob, boss, other] => {
                        let caster = u16::try_from(*caster)
                            .ok()
                            .filter(|mask| *mask > 0 && *mask & !MapActorType::All.as_flag() == 0)
                            .ok_or("Invalid skill damage caster")?;
                        let rates = [*pc, *mob, *boss, *other].map(|rate| rate.clamp(-100, 100000));
                        if *skill == 0 {
                            self.skill_damage_caster |= caster;
                            self.skill_damage = rates;
                        } else {
                            let skill = u32::try_from(*skill).map_err(|_| "Invalid skill damage skill")?;
                            self.specific_skill_damage.insert(skill, (caster, rates));
                        }
                    }
                    _ => return Err("Skill damage requires rates and a target or caster".into()),
                }
            } else {
                self.skill_damage = [0; 4];
                self.skill_damage_caster = 0;
                self.specific_skill_damage.clear();
            }
        }
        if flag == MapFlag::SkillDuration {
            if enabled {
                let [id, rate] = arguments else {
                    return Err("Skill duration requires a skill and rate".into());
                };
                let id = u32::try_from(*id).ok().filter(|id| *id > 0).ok_or("Invalid skill duration skill")?;
                let rate = u16::try_from(*rate).map_err(|_| "Invalid skill duration rate")?;
                self.skill_duration.insert(id, rate);
            } else {
                self.skill_duration.clear();
            }
        }
        if enabled {
            if flag == MapFlag::Pvp {
                for other in [MapFlag::Gvg, MapFlag::GvgCastle, MapFlag::GvgDungeon, MapFlag::Battleground] {
                    self.values.remove(&other);
                }
            } else if flag == MapFlag::Battleground {
                for other in [MapFlag::Pvp, MapFlag::Gvg, MapFlag::GvgCastle, MapFlag::GvgDungeon] {
                    self.values.remove(&other);
                }
            } else if [MapFlag::Gvg, MapFlag::GvgCastle, MapFlag::GvgDungeon].contains(&flag) {
                self.values.remove(&MapFlag::Pvp);
                self.values.remove(&MapFlag::Battleground);
            }
            if flag == MapFlag::NoBaseExp && self.get(MapFlag::Bexp, None) != 100 {
                self.values.insert(MapFlag::Bexp, 0);
            }
            if flag == MapFlag::NoJobExp && self.get(MapFlag::Jexp, None) != 100 {
                self.values.insert(MapFlag::Jexp, 0);
            }
            if flag == MapFlag::Bexp {
                self.values.remove(&MapFlag::NoBaseExp);
            }
            if flag == MapFlag::Jexp {
                self.values.remove(&MapFlag::NoJobExp);
            }
            if flag == MapFlag::NoSave {
                self.save = Some(("SavePoint".into(), 0, 0));
            }
            if flag == MapFlag::PvpNightmareDrop {
                let (id, location, rate) = match arguments {
                    [] => (-1, NightmareDropLocation::Equipped.as_flag(), 300),
                    [id, location, rate] => {
                        if *id != -1 && *id <= 0 {
                            return Err("Invalid nightmare drop item".into());
                        }
                        let location = u8::try_from(*location)
                            .ok()
                            .filter(|value| {
                                [
                                    NightmareDropLocation::Inventory.as_flag(),
                                    NightmareDropLocation::Equipped.as_flag(),
                                    NightmareDropLocation::All.as_flag(),
                                ]
                                .contains(value)
                            })
                            .ok_or("Invalid nightmare drop location")?;
                        let rate = u16::try_from(*rate)
                            .ok()
                            .filter(|rate| *rate <= 10000)
                            .ok_or("Invalid nightmare drop rate")?;
                        (*id, location, rate)
                    }
                    _ => return Err("Nightmare drop requires an item, location and rate".into()),
                };
                if self.nightmare_drops.len() >= 10 {
                    return Err("Too many nightmare drop entries".into());
                }
                self.nightmare_drops.push((id, location, rate));
            }
        } else {
            if flag == MapFlag::NoSave {
                self.save = None;
            }
        }
        let value = if !enabled {
            0
        } else if flag == MapFlag::Battleground {
            arguments.first().copied().filter(|value| (1..=2).contains(value)).unwrap_or(1)
        } else if matches!(
            flag,
            MapFlag::Bexp | MapFlag::Jexp | MapFlag::SpecialPopup | MapFlag::InvincibleTime
        ) {
            arguments.first().copied().unwrap_or(0)
        } else if flag == MapFlag::NoCommand {
            arguments.first().copied().filter(|value| *value > 0).unwrap_or(100)
        } else {
            1
        };
        self.values.insert(flag, value);
        Ok(())
    }

    pub fn skill_damage_rate(&self, skill_id: u32, caster: MapActorType, target_kind: usize) -> i32 {
        if !self.enabled(MapFlag::SkillDamage) || target_kind >= 4 {
            return 0;
        }
        let general = if self.skill_damage_caster & caster.as_flag() != 0 {
            self.skill_damage[target_kind]
        } else {
            0
        };
        let specific = self
            .specific_skill_damage
            .get(&skill_id)
            .filter(|(mask, _)| *mask & caster.as_flag() != 0)
            .map_or(0, |(_, rates)| rates[target_kind]);
        general.saturating_add(specific)
    }

    pub(crate) fn skill_duration_rate(&self, skill_id: u32) -> Option<u16> {
        self.enabled(MapFlag::SkillDuration)
            .then(|| self.skill_duration.get(&skill_id).copied())
            .flatten()
    }
}
#[derive(Deserialize)]
pub struct MapFlagDefinition {
    pub map: String,
    pub flag: String,
    #[serde(default = "enabled_default")]
    pub enabled: bool,
    #[serde(default)]
    pub arguments: Vec<i32>,
    #[serde(default)]
    pub save: Option<(String, u16, u16)>,
}
fn enabled_default() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reset_permits_character_resets_without_removing_map_rules() {
        let mut flags = MapFlags::default();
        flags.set(MapFlag::NoTeleport, true, &[]).unwrap();
        flags.set(MapFlag::Reset, true, &[]).unwrap();
        assert!(flags.enabled(MapFlag::Reset));
        assert!(flags.enabled(MapFlag::NoTeleport));
        flags.set(MapFlag::Reset, false, &[]).unwrap();
        assert!(!flags.enabled(MapFlag::Reset));
        assert!(flags.enabled(MapFlag::NoTeleport));
    }

    #[test]
    fn combat_modes_replace_incompatible_modes_and_castles_require_active_siege() {
        let mut flags = MapFlags::default();
        flags.set(MapFlag::GvgCastle, true, &[]).unwrap();
        assert!(flags.is_gvg());
        assert!(!flags.versus(false));
        assert!(flags.versus(true));
        flags.set(MapFlag::Pvp, true, &[]).unwrap();
        assert!(!flags.is_gvg());
        assert!(flags.versus(false));
        flags.set(MapFlag::Battleground, true, &[2]).unwrap();
        assert!(!flags.enabled(MapFlag::Pvp));
        assert_eq!(flags.get(MapFlag::Battleground, None), 2);
        flags.set(MapFlag::Gvg, true, &[]).unwrap();
        assert!(!flags.enabled(MapFlag::Battleground));
        assert!(flags.enabled(MapFlag::Gvg));
        flags.set(MapFlag::Battleground, true, &[9]).unwrap();
        assert!(!flags.is_gvg());
        assert_eq!(flags.get(MapFlag::Battleground, None), 1);
    }

    #[test]
    fn exp_and_loot_combinations_preserve_independent_rules_and_rate_conflicts() {
        let mut flags = MapFlags::default();
        flags.set(MapFlag::Bexp, true, &[250]).unwrap();
        flags.set(MapFlag::NoBaseExp, true, &[]).unwrap();
        assert_eq!(flags.get(MapFlag::Bexp, None), 0);
        assert!(flags.enabled(MapFlag::NoBaseExp));
        assert!(!flags.enabled(MapFlag::NoExp));
        flags.set(MapFlag::Bexp, true, &[175]).unwrap();
        assert!(!flags.enabled(MapFlag::NoBaseExp));
        assert_eq!(flags.get(MapFlag::Bexp, None), 175);
        flags.set(MapFlag::NoExp, true, &[]).unwrap();
        flags.set(MapFlag::NoLoot, true, &[]).unwrap();
        assert!(flags.enabled(MapFlag::NoExp));
        assert!(flags.enabled(MapFlag::NoLoot));
        flags.set(MapFlag::NoMvpLoot, false, &[]).unwrap();
        assert!(flags.enabled(MapFlag::NoMobLoot));
        assert!(!flags.enabled(MapFlag::NoLoot));
    }

    #[test]
    fn restricted_zones_accumulate_and_removing_one_keeps_the_others() {
        let mut flags = MapFlags::default();
        flags.set(MapFlag::Restricted, true, &[1]).unwrap();
        flags.set(MapFlag::Restricted, true, &[7]).unwrap();
        assert_eq!(
            flags.get(MapFlag::Restricted, None) as u32,
            MapRestrictionZone::Zone1.as_flag() | MapRestrictionZone::Zone7.as_flag()
        );
        flags.set(MapFlag::Restricted, false, &[1]).unwrap();
        assert_eq!(flags.get(MapFlag::Restricted, None) as u32, MapRestrictionZone::Zone7.as_flag());
        flags.set(MapFlag::Restricted, false, &[1]).unwrap();
        assert!(flags.enabled(MapFlag::Restricted));
        flags.set(MapFlag::Restricted, false, &[]).unwrap();
        assert!(!flags.enabled(MapFlag::Restricted));
        assert!(flags.set(MapFlag::Restricted, true, &[28]).is_err());
    }

    #[test]
    fn skill_adjustments_select_caster_target_and_specific_skill_and_clear_on_removal() {
        let mut flags = MapFlags::default();
        flags
            .set(MapFlag::SkillDamage, true, &[
                0,
                i32::from(MapActorType::All.as_flag()),
                10,
                20,
                30,
                40,
            ])
            .unwrap();
        flags
            .set(MapFlag::SkillDamage, true, &[
                19,
                i32::from(MapActorType::Player.as_flag()),
                -20,
                100,
                0,
                0,
            ])
            .unwrap();
        assert_eq!(flags.get(MapFlag::SkillDamage, Some(5)), i32::from(MapActorType::All.as_flag()));
        assert_eq!(flags.skill_damage_rate(19, MapActorType::Player, 0), -10);
        assert_eq!(flags.skill_damage_rate(19, MapActorType::Player, 1), 120);
        assert_eq!(flags.skill_damage_rate(19, MapActorType::Monster, 1), 20);
        assert_eq!(flags.skill_damage_rate(20, MapActorType::Player, 1), 20);
        flags.set(MapFlag::SkillDamage, false, &[]).unwrap();
        assert_eq!(flags.skill_damage_rate(19, MapActorType::Player, 1), 0);
        flags.set(MapFlag::SkillDuration, true, &[115, 400]).unwrap();
        assert_eq!(flags.get(MapFlag::SkillDuration, Some(115)), 400);
        flags.set(MapFlag::SkillDuration, false, &[]).unwrap();
        assert_eq!(flags.get(MapFlag::SkillDuration, Some(115)), 100);
    }

    #[test]
    fn nightmare_drop_entries_preserve_their_item_location_and_per_ten_thousand_rate() {
        let mut flags = MapFlags::default();
        flags.set(MapFlag::PvpNightmareDrop, true, &[]).unwrap();
        flags
            .set(MapFlag::PvpNightmareDrop, true, &[
                501,
                i32::from(NightmareDropLocation::Inventory.as_flag()),
                10000,
            ])
            .unwrap();
        assert_eq!(flags.nightmare_drops, vec![
            (-1, NightmareDropLocation::Equipped.as_flag(), 300),
            (501, NightmareDropLocation::Inventory.as_flag(), 10000)
        ]);
        assert!(flags.set(MapFlag::PvpNightmareDrop, true, &[501, 2, 10001]).is_err());
    }
}
