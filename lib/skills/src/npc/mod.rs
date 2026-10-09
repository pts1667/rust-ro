#![allow(dead_code, unused_must_use, unused_imports, unused_variables)]

use models::enums::skill_enums::SkillEnum;
use crate::Skill;

pub mod npc_piercingatt;
pub use npc_piercingatt::*;
pub mod npc_mentalbreaker;
pub use npc_mentalbreaker::*;
pub mod npc_rangeattack;
pub use npc_rangeattack::*;
pub mod npc_attrichange;
pub use npc_attrichange::*;
pub mod npc_changewater;
pub use npc_changewater::*;
pub mod npc_changeground;
pub use npc_changeground::*;
pub mod npc_changefire;
pub use npc_changefire::*;
pub mod npc_changewind;
pub use npc_changewind::*;
pub mod npc_changepoison;
pub use npc_changepoison::*;
pub mod npc_changeholy;
pub use npc_changeholy::*;
pub mod npc_changedarkness;
pub use npc_changedarkness::*;
pub mod npc_changetelekinesis;
pub use npc_changetelekinesis::*;
pub mod npc_criticalslash;
pub use npc_criticalslash::*;
pub mod npc_comboattack;
pub use npc_comboattack::*;
pub mod npc_guidedattack;
pub use npc_guidedattack::*;
pub mod npc_selfdestruction;
pub use npc_selfdestruction::*;
pub mod npc_splashattack;
pub use npc_splashattack::*;
pub mod npc_suicide;
pub use npc_suicide::*;
pub mod npc_poison;
pub use npc_poison::*;
pub mod npc_blindattack;
pub use npc_blindattack::*;
pub mod npc_silenceattack;
pub use npc_silenceattack::*;
pub mod npc_stunattack;
pub use npc_stunattack::*;
pub mod npc_petrifyattack;
pub use npc_petrifyattack::*;
pub mod npc_curseattack;
pub use npc_curseattack::*;
pub mod npc_sleepattack;
pub use npc_sleepattack::*;
pub mod npc_randomattack;
pub use npc_randomattack::*;
pub mod npc_waterattack;
pub use npc_waterattack::*;
pub mod npc_groundattack;
pub use npc_groundattack::*;
pub mod npc_fireattack;
pub use npc_fireattack::*;
pub mod npc_windattack;
pub use npc_windattack::*;
pub mod npc_poisonattack;
pub use npc_poisonattack::*;
pub mod npc_holyattack;
pub use npc_holyattack::*;
pub mod npc_darknessattack;
pub use npc_darknessattack::*;
pub mod npc_telekinesisattack;
pub use npc_telekinesisattack::*;
pub mod npc_magicalattack;
pub use npc_magicalattack::*;
pub mod npc_metamorphosis;
pub use npc_metamorphosis::*;
pub mod npc_provocation;
pub use npc_provocation::*;
pub mod npc_smoking;
pub use npc_smoking::*;
pub mod npc_summonslave;
pub use npc_summonslave::*;
pub mod npc_emotion;
pub use npc_emotion::*;
pub mod npc_transformation;
pub use npc_transformation::*;
pub mod npc_blooddrain;
pub use npc_blooddrain::*;
pub mod npc_energydrain;
pub use npc_energydrain::*;
pub mod npc_keeping;
pub use npc_keeping::*;
pub mod npc_darkbreath;
pub use npc_darkbreath::*;
pub mod npc_darkblessing;
pub use npc_darkblessing::*;
pub mod npc_barrier;
pub use npc_barrier::*;
pub mod npc_defender;
pub use npc_defender::*;
pub mod npc_lick;
pub use npc_lick::*;
pub mod npc_hallucination;
pub use npc_hallucination::*;
pub mod npc_rebirth;
pub use npc_rebirth::*;
pub mod npc_summonmonster;
pub use npc_summonmonster::*;
pub mod sa_monocell;
pub use sa_monocell::*;
pub mod sa_classchange;
pub use sa_classchange::*;
pub mod sa_summonmonster;
pub use sa_summonmonster::*;
pub mod sa_reverseorcish;
pub use sa_reverseorcish::*;
pub mod sa_death;
pub use sa_death::*;
pub mod sa_fortune;
pub use sa_fortune::*;
pub mod sa_tamingmonster;
pub use sa_tamingmonster::*;
pub mod sa_question;
pub use sa_question::*;
pub mod sa_gravity;
pub use sa_gravity::*;
pub mod sa_levelup;
pub use sa_levelup::*;
pub mod sa_instantdeath;
pub use sa_instantdeath::*;
pub mod sa_fullrecovery;
pub use sa_fullrecovery::*;
pub mod sa_coma;
pub use sa_coma::*;
pub mod npc_randommove;
pub use npc_randommove::*;
pub mod npc_speedup;
pub use npc_speedup::*;
pub mod npc_revenge;
pub use npc_revenge::*;
pub mod itm_tomahawk;
pub use itm_tomahawk::*;
pub mod npc_darkcross;
pub use npc_darkcross::*;
pub mod npc_granddarkness;
pub use npc_granddarkness::*;
pub mod npc_darkstrike;
pub use npc_darkstrike::*;
pub mod npc_darkthunder;
pub use npc_darkthunder::*;
pub mod npc_stop;
pub use npc_stop::*;
pub mod npc_weaponbraker;
pub use npc_weaponbraker::*;
pub mod npc_armorbrake;
pub use npc_armorbrake::*;
pub mod npc_helmbrake;
pub use npc_helmbrake::*;
pub mod npc_shieldbrake;
pub use npc_shieldbrake::*;
pub mod npc_undeadattack;
pub use npc_undeadattack::*;
pub mod npc_changeundead;
pub use npc_changeundead::*;
pub mod npc_powerup;
pub use npc_powerup::*;
pub mod npc_agiup;
pub use npc_agiup::*;
pub mod npc_siegemode;
pub use npc_siegemode::*;
pub mod npc_callslave;
pub use npc_callslave::*;
pub mod npc_invisible;
pub use npc_invisible::*;
pub mod npc_run;
pub use npc_run::*;
pub mod npc_emotion_on;
pub use npc_emotion_on::*;
pub mod item_enchantarms;
pub use item_enchantarms::*;
pub mod npc_earthquake;
pub use npc_earthquake::*;
pub mod npc_firebreath;
pub use npc_firebreath::*;
pub mod npc_icebreath;
pub use npc_icebreath::*;
pub mod npc_thunderbreath;
pub use npc_thunderbreath::*;
pub mod npc_acidbreath;
pub use npc_acidbreath::*;
pub mod npc_darknessbreath;
pub use npc_darknessbreath::*;
pub mod npc_dragonfear;
pub use npc_dragonfear::*;
pub mod npc_bleeding;
pub use npc_bleeding::*;
pub mod npc_pulsestrike;
pub use npc_pulsestrike::*;
pub mod npc_helljudgement;
pub use npc_helljudgement::*;
pub mod npc_widesilence;
pub use npc_widesilence::*;
pub mod npc_widefreeze;
pub use npc_widefreeze::*;
pub mod npc_widebleeding;
pub use npc_widebleeding::*;
pub mod npc_widestone;
pub use npc_widestone::*;
pub mod npc_wideconfuse;
pub use npc_wideconfuse::*;
pub mod npc_widesleep;
pub use npc_widesleep::*;
pub mod npc_widesight;
pub use npc_widesight::*;
pub mod npc_evilland;
pub use npc_evilland::*;
pub mod npc_magicmirror;
pub use npc_magicmirror::*;
pub mod npc_slowcast;
pub use npc_slowcast::*;
pub mod npc_criticalwound;
pub use npc_criticalwound::*;
pub mod npc_expulsion;
pub use npc_expulsion::*;
pub mod npc_stoneskin;
pub use npc_stoneskin::*;
pub mod npc_antimagic;
pub use npc_antimagic::*;
pub mod npc_widecurse;
pub use npc_widecurse::*;
pub mod npc_widestun;
pub use npc_widestun::*;
pub mod npc_vampire_gift;
pub use npc_vampire_gift::*;
pub mod npc_widesouldrain;
pub use npc_widesouldrain::*;
pub mod npc_talk;
pub use npc_talk::*;
pub mod npc_hellpower;
pub use npc_hellpower::*;
pub mod npc_widehelldignity;
pub use npc_widehelldignity::*;
pub mod npc_invincible;
pub use npc_invincible::*;
pub mod npc_invincibleoff;
pub use npc_invincibleoff::*;
pub mod npc_allheal;
pub use npc_allheal::*;
pub mod cash_blessing;
pub use cash_blessing::*;
pub mod cash_incagi;
pub use cash_incagi::*;
pub mod cash_assumptio;
pub use cash_assumptio::*;
pub mod all_catcry;
pub use all_catcry::*;
pub mod all_partyflee;
pub use all_partyflee::*;
pub mod all_dream_summernight;
pub use all_dream_summernight::*;
pub mod all_reverseorcish;
pub use all_reverseorcish::*;
pub mod all_wewish;
pub use all_wewish::*;
pub mod npc_venomfog;
pub use npc_venomfog::*;
pub mod npc_comet;
pub use npc_comet::*;
pub mod npc_maxpain;
pub use npc_maxpain::*;
pub mod npc_maxpain_atk;
pub use npc_maxpain_atk::*;
pub mod npc_jackfrost;
pub use npc_jackfrost::*;
pub mod npc_reverberation;
pub use npc_reverberation::*;
pub mod npc_reverberation_atk;
pub use npc_reverberation_atk::*;
pub mod npc_lex_aeterna;
pub use npc_lex_aeterna::*;
pub mod npc_arrowstorm;
pub use npc_arrowstorm::*;
pub mod npc_cloud_kill;
pub use npc_cloud_kill::*;
pub mod npc_ignitionbreak;
pub use npc_ignitionbreak::*;
pub mod npc_phantomthrust;
pub use npc_phantomthrust::*;
pub mod npc_poison_buster;
pub use npc_poison_buster::*;
pub mod npc_hallucinationwalk;
pub use npc_hallucinationwalk::*;
pub mod npc_electricwalk;
pub use npc_electricwalk::*;
pub mod npc_firewalk;
pub use npc_firewalk::*;

pub fn to_object(skill_enum: SkillEnum, level: u8) -> Option<Box<dyn Skill>> {
    match skill_enum {
        SkillEnum::NpcPiercingatt => NpcPiercingatt::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcMentalbreaker => NpcMentalbreaker::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcRangeattack => NpcRangeattack::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcAttrichange => NpcAttrichange::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcChangewater => NpcChangewater::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcChangeground => NpcChangeground::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcChangefire => NpcChangefire::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcChangewind => NpcChangewind::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcChangepoison => NpcChangepoison::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcChangeholy => NpcChangeholy::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcChangedarkness => NpcChangedarkness::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcChangetelekinesis => NpcChangetelekinesis::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcCriticalslash => NpcCriticalslash::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcComboattack => NpcComboattack::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcGuidedattack => NpcGuidedattack::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcSelfdestruction => NpcSelfdestruction::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcSplashattack => NpcSplashattack::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcSuicide => NpcSuicide::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcPoison => NpcPoison::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcBlindattack => NpcBlindattack::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcSilenceattack => NpcSilenceattack::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcStunattack => NpcStunattack::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcPetrifyattack => NpcPetrifyattack::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcCurseattack => NpcCurseattack::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcSleepattack => NpcSleepattack::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcRandomattack => NpcRandomattack::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcWaterattack => NpcWaterattack::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcGroundattack => NpcGroundattack::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcFireattack => NpcFireattack::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcWindattack => NpcWindattack::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcPoisonattack => NpcPoisonattack::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcHolyattack => NpcHolyattack::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcDarknessattack => NpcDarknessattack::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcTelekinesisattack => NpcTelekinesisattack::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcMagicalattack => NpcMagicalattack::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcMetamorphosis => NpcMetamorphosis::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcProvocation => NpcProvocation::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcSmoking => NpcSmoking::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcSummonslave => NpcSummonslave::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcEmotion => NpcEmotion::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcTransformation => NpcTransformation::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcBlooddrain => NpcBlooddrain::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcEnergydrain => NpcEnergydrain::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcKeeping => NpcKeeping::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcDarkbreath => NpcDarkbreath::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcDarkblessing => NpcDarkblessing::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcBarrier => NpcBarrier::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcDefender => NpcDefender::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcLick => NpcLick::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcHallucination => NpcHallucination::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcRebirth => NpcRebirth::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcSummonmonster => NpcSummonmonster::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::SaMonocell => SaMonocell::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::SaClasschange => SaClasschange::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::SaSummonmonster => SaSummonmonster::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::SaReverseorcish => SaReverseorcish::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::SaDeath => SaDeath::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::SaFortune => SaFortune::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::SaTamingmonster => SaTamingmonster::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::SaQuestion => SaQuestion::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::SaGravity => SaGravity::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::SaLevelup => SaLevelup::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::SaInstantdeath => SaInstantdeath::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::SaFullrecovery => SaFullrecovery::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::SaComa => SaComa::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcRandommove => NpcRandommove::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcSpeedup => NpcSpeedup::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcRevenge => NpcRevenge::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::ItmTomahawk => ItmTomahawk::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcDarkcross => NpcDarkcross::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcGranddarkness => NpcGranddarkness::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcDarkstrike => NpcDarkstrike::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcDarkthunder => NpcDarkthunder::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcStop => NpcStop::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcWeaponbraker => NpcWeaponbraker::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcArmorbrake => NpcArmorbrake::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcHelmbrake => NpcHelmbrake::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcShieldbrake => NpcShieldbrake::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcUndeadattack => NpcUndeadattack::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcChangeundead => NpcChangeundead::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcPowerup => NpcPowerup::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcAgiup => NpcAgiup::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcSiegemode => NpcSiegemode::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcCallslave => NpcCallslave::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcInvisible => NpcInvisible::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcRun => NpcRun::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcEmotionOn => NpcEmotionOn::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::ItemEnchantarms => ItemEnchantarms::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcEarthquake => NpcEarthquake::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcFirebreath => NpcFirebreath::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcIcebreath => NpcIcebreath::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcThunderbreath => NpcThunderbreath::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcAcidbreath => NpcAcidbreath::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcDarknessbreath => NpcDarknessbreath::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcDragonfear => NpcDragonfear::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcBleeding => NpcBleeding::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcPulsestrike => NpcPulsestrike::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcHelljudgement => NpcHelljudgement::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcWidesilence => NpcWidesilence::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcWidefreeze => NpcWidefreeze::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcWidebleeding => NpcWidebleeding::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcWidestone => NpcWidestone::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcWideconfuse => NpcWideconfuse::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcWidesleep => NpcWidesleep::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcWidesight => NpcWidesight::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcEvilland => NpcEvilland::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcMagicmirror => NpcMagicmirror::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcSlowcast => NpcSlowcast::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcCriticalwound => NpcCriticalwound::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcExpulsion => NpcExpulsion::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcStoneskin => NpcStoneskin::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcAntimagic => NpcAntimagic::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcWidecurse => NpcWidecurse::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcWidestun => NpcWidestun::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcVampireGift => NpcVampireGift::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcWidesouldrain => NpcWidesouldrain::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcTalk => NpcTalk::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcHellpower => NpcHellpower::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcWidehelldignity => NpcWidehelldignity::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcInvincible => NpcInvincible::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcInvincibleoff => NpcInvincibleoff::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcAllheal => NpcAllheal::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::CashBlessing => CashBlessing::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::CashIncagi => CashIncagi::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::CashAssumptio => CashAssumptio::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::AllCatcry => AllCatcry::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::AllPartyflee => AllPartyflee::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::AllDreamSummernight => AllDreamSummernight::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::AllReverseorcish => AllReverseorcish::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::AllWewish => AllWewish::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcVenomfog => NpcVenomfog::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcComet => NpcComet::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcMaxpain => NpcMaxpain::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcMaxpainAtk => NpcMaxpainAtk::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcJackfrost => NpcJackfrost::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcReverberation => NpcReverberation::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcReverberationAtk => NpcReverberationAtk::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcLexAeterna => NpcLexAeterna::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcArrowstorm => NpcArrowstorm::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcCloudKill => NpcCloudKill::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcIgnitionbreak => NpcIgnitionbreak::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcPhantomthrust => NpcPhantomthrust::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcPoisonBuster => NpcPoisonBuster::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcHallucinationwalk => NpcHallucinationwalk::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcElectricwalk => NpcElectricwalk::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        SkillEnum::NpcFirewalk => NpcFirewalk::new(level).map(|s| Box::new(s) as Box<dyn Skill>),
        _ => None,
    }
}
