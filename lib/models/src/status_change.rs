use enum_macro::WithMaskValueU32;
use serde::{Deserialize, Serialize};

use crate::enums::bonus::BonusType;
use crate::enums::element::Element;
use crate::enums::mob::MobRace;
use crate::enums::status::StatusEffect;
use crate::enums::{EnumWithMaskValueU32, EnumWithNumberValue, EnumWithStringValue};

macro_rules! status_changes {
    ($( $kind:ident = $id:literal => $name:literal ),+ $(,)?) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[repr(u16)]
        pub enum StatusChangeKind { $( $kind = $id, )+ }

        impl StatusChangeKind {
            pub fn from_id(id: i32) -> Option<Self> {
                match id { $( $id => Some(Self::$kind), )+ _ => None }
            }

            pub fn from_name(name: &str) -> Option<Self> {
                let name = name.to_ascii_uppercase();
                let name = name.strip_prefix("SC_").unwrap_or(&name);
                match name { $( $name => Some(Self::$kind), )+ _ => None }
            }

            pub fn name(self) -> &'static str {
                match self { $( Self::$kind => concat!("SC_", $name), )+ }
            }

            pub fn id(self) -> u16 { self as u16 }
        }
    };
}

status_changes! {
    Stone = 0 => "STONE", Freeze = 1 => "FREEZE", Stun = 2 => "STUN",
    Sleep = 3 => "SLEEP", Poison = 4 => "POISON", Curse = 5 => "CURSE",
    Silence = 6 => "SILENCE", Confusion = 7 => "CONFUSION", Blind = 8 => "BLIND",
    Bleeding = 9 => "BLEEDING", DeadlyPoison = 10 => "DPOISON",
    Provoke = 20 => "PROVOKE", Endure = 21 => "ENDURE", TwoHandQuicken = 22 => "TWOHANDQUICKEN",
    Concentrate = 23 => "CONCENTRATE", Hiding = 24 => "HIDING", Cloaking = 25 => "CLOAKING",
    EnchantPoison = 26 => "ENCPOISON", Quagmire = 28 => "QUAGMIRE", Angelus = 29 => "ANGELUS",
    Blessing = 30 => "BLESSING", IncreaseAgi = 32 => "INCREASEAGI", DecreaseAgi = 33 => "DECREASEAGI",
    SlowPoison = 34 => "SLOWPOISON", Impositio = 35 => "IMPOSITIO", Suffragium = 36 => "SUFFRAGIUM",
    Aspersio = 37 => "ASPERSIO", Benedictio = 38 => "BENEDICTIO", Kyrie = 39 => "KYRIE",
    Magnificat = 40 => "MAGNIFICAT", Gloria = 41 => "GLORIA", LexAeterna = 42 => "AETERNA",
    Adrenaline = 43 => "ADRENALINE", WeaponPerfection = 44 => "WEAPONPERFECTION", Overthrust = 45 => "OVERTHRUST",
    MaximizePower = 46 => "MAXIMIZEPOWER", TrickDead = 47 => "TRICKDEAD", Loud = 48 => "LOUD",
    EnergyCoat = 49 => "ENERGYCOAT", Hallucination = 52 => "HALLUCINATION",
    AspdPotion0 = 55 => "ASPDPOTION0", AspdPotion1 = 56 => "ASPDPOTION1", AspdPotion2 = 57 => "ASPDPOTION2",
    AspdPotion3 = 58 => "ASPDPOTION3", SpeedUp0 = 59 => "SPEEDUP0", SpeedUp1 = 60 => "SPEEDUP1",
    AttackPotion = 61 => "ATKPOTION", MagicAttackPotion = 62 => "MATKPOTION", Wedding = 63 => "WEDDING",
    SlowDown = 64 => "SLOWDOWN", Sight = 88 => "SIGHT", Ruwach = 90 => "RUWACH",
    FireWeapon = 96 => "FIREWEAPON", WaterWeapon = 97 => "WATERWEAPON", WindWeapon = 98 => "WINDWEAPON",
    EarthWeapon = 99 => "EARTHWEAPON", ArmorElementWater = 105 => "ARMOR_ELEMENT_WATER",
    Concentration = 110 => "CONCENTRATION", Assumptio = 115 => "ASSUMPTIO", WindWalk = 121 => "WINDWALK",
    ChangeUndead = 128 => "CHANGEUNDEAD", SteelBody = 136 => "STEELBODY", Orcish = 137 => "ORCISH",
    ShadowWeapon = 144 => "SHADOWWEAPON", GhostWeapon = 146 => "GHOSTWEAPON", Coma = 185 => "COMA",
    Intravision = 186 => "INTRAVISION", IncAllStatus = 187 => "INCALLSTATUS", IncStr = 188 => "INCSTR",
    IncAgi = 189 => "INCAGI", IncVit = 190 => "INCVIT", IncInt = 191 => "INCINT", IncDex = 192 => "INCDEX",
    IncLuk = 193 => "INCLUK", IncHit = 194 => "INCHIT", IncHitRate = 195 => "INCHITRATE",
    IncFlee = 196 => "INCFLEE", IncFleeRate = 197 => "INCFLEERATE", IncMaxHpRate = 198 => "INCMHPRATE",
    IncMaxSpRate = 199 => "INCMSPRATE", IncAttackRate = 200 => "INCATKRATE", IncMagicAttackRate = 201 => "INCMATKRATE",
    IncDefRate = 202 => "INCDEFRATE", StrFood = 203 => "STRFOOD", AgiFood = 204 => "AGIFOOD",
    VitFood = 205 => "VITFOOD", IntFood = 206 => "INTFOOD", DexFood = 207 => "DEXFOOD", LukFood = 208 => "LUKFOOD",
    HitFood = 209 => "HITFOOD", FleeFood = 210 => "FLEEFOOD", BaseAttackFood = 211 => "BATKFOOD",
    WeaponAttackFood = 212 => "WATKFOOD", MagicAttackFood = 213 => "MATKFOOD", StatusResistance = 214 => "SCRESIST",
    Christmas = 215 => "XMAS", EnchantArms = 250 => "ENCHANTARMS", ArmorChange = 252 => "ARMORCHANGE",
    CriticalWound = 253 => "CRITICALWOUND", SlowCast = 255 => "SLOWCAST", Summer = 256 => "SUMMER",
    ExpBoost = 257 => "EXPBOOST", ItemBoost = 258 => "ITEMBOOST", BossMapInfo = 259 => "BOSSMAPINFO",
    LifeInsurance = 260 => "LIFEINSURANCE", IncCritical = 261 => "INCCRI", MdefRate = 265 => "MDEF_RATE",
    IncHealRate = 267 => "INCHEALRATE", Pneuma = 268 => "PNEUMA", ArmorResist = 271 => "ARMOR_RESIST",
    SpCostRate = 272 => "SPCOST_RATE", CommonStatusResist = 273 => "COMMONSC_RESIST", DefRate = 275 => "DEF_RATE",
    WalkSpeed = 277 => "WALKSPEED", MercFleeUp = 278 => "MERC_FLEEUP", MercAttackUp = 279 => "MERC_ATKUP",
    MercHpUp = 280 => "MERC_HPUP", MercSpUp = 281 => "MERC_SPUP", MercHitUp = 282 => "MERC_HITUP",
    MercQuicken = 283 => "MERC_QUICKEN", ManuAttack = 297 => "MANU_ATK", ManuDef = 298 => "MANU_DEF",
    SplAttack = 299 => "SPL_ATK", SplDef = 300 => "SPL_DEF", ManuMagicAttack = 301 => "MANU_MATK",
    SplMagicAttack = 302 => "SPL_MATK", CashStrFood = 303 => "FOOD_STR_CASH", CashAgiFood = 304 => "FOOD_AGI_CASH",
    CashVitFood = 305 => "FOOD_VIT_CASH", CashDexFood = 306 => "FOOD_DEX_CASH", CashIntFood = 307 => "FOOD_INT_CASH",
    CashLukFood = 308 => "FOOD_LUK_CASH", ArmorElementEarth = 669 => "ARMOR_ELEMENT_EARTH",
    ArmorElementFire = 670 => "ARMOR_ELEMENT_FIRE", ArmorElementWind = 671 => "ARMOR_ELEMENT_WIND",
    IncreaseMaxHp = 736 => "INCREASE_MAXHP", IncreaseMaxSp = 737 => "INCREASE_MAXSP",
    NoRecovery = 627 => "NORECOVER_STATE", DefSet = 556 => "DEFSET", MdefSet = 557 => "MDEFSET",
    AutoGuard = 76 => "AUTOGUARD", MagicMirror = 254 => "MAGICMIRROR", PartyFlee = 528 => "PARTYFLEE",
    HellPower = 294 => "HELLPOWER", SignumCrucis = 31 => "SIGNUMCRUCIS", AuraBlade = 108 => "AURABLADE",
    ExplosionSpirits = 92 => "EXPLOSIONSPIRITS", Deluge = 101 => "DELUGE", AutoSpell = 83 => "AUTOSPELL", Volcano = 100 => "VOLCANO", ViolentGale = 102 => "VIOLENTGALE",
    WeaponAttackElement = 103 => "WATK_ELEMENT", Nen = 237 => "NEN",
    MindBreaker = 130 => "MINDBREAKER", Kaizel = 147 => "KAIZEL", Kaahi = 148 => "KAAHI", Kaupe = 149 => "KAUPE",
    Ske = 222 => "SKE", Kaite = 223 => "KAITE", Swoo = 224 => "SWOO", Ska = 225 => "SKA",
    ReadyStorm = 138 => "READYSTORM", ReadyDown = 139 => "READYDOWN", ReadyTurn = 140 => "READYTURN", ReadyCounter = 141 => "READYCOUNTER",
    Dodge = 142 => "DODGE", SevenWind = 274 => "SEVENWIND",
    Dancing = 164 => "DANCING", Longing = 157 => "LONGING", EternalChaos = 167 => "ETERNALCHAOS", DrumBattle = 168 => "DRUMBATTLE",
    Nibelungen = 169 => "NIBELUNGEN", Siegfried = 172 => "SIEGFRIED", Whistle = 173 => "WHISTLE", AssnCros = 174 => "ASSNCROS",
    PoemBragi = 175 => "POEMBRAGI", AppleIdun = 176 => "APPLEIDUN", Humming = 178 => "HUMMING", DontForgetMe = 179 => "DONTFORGETME",
    Fortune = 180 => "FORTUNE", Service4U = 181 => "SERVICE4U", RokisWeil = 170 => "ROKISWEIL", IntoAbyss = 171 => "INTOABYSS",
    Run = 143 => "RUN", Spurt = 183 => "SPURT", Ankle = 65 => "ANKLE",
    StripWeapon = 68 => "STRIPWEAPON", StripShield = 69 => "STRIPSHIELD", StripArmor = 70 => "STRIPARMOR", StripHelm = 71 => "STRIPHELM",
    ProtectWeapon = 72 => "CP_WEAPON", ProtectShield = 73 => "CP_SHIELD", ProtectArmor = 74 => "CP_ARMOR", ProtectHelm = 75 => "CP_HELM", MagicRod = 81 => "MAGICROD",
    JointBeat = 129 => "JOINTBEAT", Splasher = 78 => "SPLASHER", Stop = 182 => "STOP", WinkCharm = 161 => "WINKCHARM",
    Spirit = 184 => "SPIRIT", CartBoost = 123 => "CARTBOOST", StoneWait = 11 => "STONEWAIT",
    HomAvoid = 241 => "AVOID", HomChange = 242 => "CHANGE", Bloodlust = 243 => "BLOODLUST", Fleet = 244 => "FLEET", HomSpeed = 245 => "SPEED", HomDefence = 246 => "DEFENCE",
    AutoBerserk = 85 => "AUTOBERSERK", Defender = 80 => "DEFENDER", Devotion = 134 => "DEVOTION", Berserk = 112 => "BERSERK", Parrying = 109 => "PARRYING", ReflectShield = 77 => "REFLECTSHIELD",
    Regeneration = 153 => "REGENERATION", Basilica = 116 => "BASILICA", ChaseWalk = 124 => "CHASEWALK", ChaseWalkStrength = 596 => "CHASEWALK2",
    Sma = 239 => "SMA", MagicalAttack = 251 => "MAGICALATTACK",
    BattleOrders = 152 => "BATTLEORDERS", Leadership = 508 => "LEADERSHIP", GloryWounds = 509 => "GLORYWOUNDS", SoulCold = 510 => "SOULCOLD", HawkEyes = 511 => "HAWKEYES",
    SafetyWall = 89 => "SAFETYWALL", BrokenArmor = 50 => "BROKENARMOR", BrokenWeapon = 51 => "BROKENWEAPON", Keeping = 66 => "KEEPING", Barrier = 67 => "BARRIER",
    Armor = 104 => "ARMOR", ElementalChange = 165 => "ELEMENTALCHANGE", ModeChange = 177 => "MODECHANGE", Rebirth = 284 => "REBIRTH",
    Invincible = 295 => "INVINCIBLE", MaxPain = 668 => "MAXPAIN", WeaponBreaker = 908 => "WEAPONBREAKER", Powerup = 951 => "POWERUP", Agiup = 952 => "AGIUP",
    SpiderWeb = 133 => "SPIDERWEB", AutoCounter = 87 => "AUTOCOUNTER",
    RichMankim = 166 => "RICHMANKIM", Hermode = 158 => "HERMODE", Marionette = 126 => "MARIONETTE", Marionette2 = 127 => "MARIONETTE2", Gospel = 114 => "GOSPEL",
    SunComfort = 217 => "SUN_COMFORT", MoonComfort = 218 => "MOON_COMFORT", StarComfort = 219 => "STAR_COMFORT", Miracle = 227 => "MIRACLE",
    Edp = 119 => "EDP", PoisonReact = 27 => "POISONREACT", RejectSword = 125 => "REJECTSWORD", Preserve = 151 => "PRESERVE",
    DoubleCast = 154 => "DOUBLECAST", Memorize = 131 => "MEMORIZE", MagicPower = 118 => "MAGICPOWER", TensionRelax = 111 => "TENSIONRELAX",
    Meltdown = 122 => "MELTDOWN", Sacrifice = 135 => "SACRIFICE", Increasing = 230 => "INCREASING", Utsusemi = 233 => "UTSUSEMI",
    Tatamigaeshi = 232 => "TATAMIGAESHI", BladeStopWait = 94 => "BLADESTOP_WAIT", BladeStop = 95 => "BLADESTOP", SightBlaster = 160 => "SIGHTBLASTER",
    CloseConfine = 162 => "CLOSECONFINE", CloseConfine2 = 163 => "CLOSECONFINE2", Shrink = 159 => "SHRINK",
    SpearQuicken = 86 => "SPEARQUICKEN", OneHand = 150 => "ONEHAND", Adrenaline2 = 145 => "ADRENALINE2", MaxOverThrust = 156 => "MAXOVERTHRUST",
    TrueSight = 120 => "TRUESIGHT", Providence = 79 => "PROVIDENCE",
    EntryQueueApplyDelay = 704 => "ENTRY_QUEUE_APPLY_DELAY", EntryQueueNotifyAdmissionTimeOut = 705 => "ENTRY_QUEUE_NOTIFY_ADMISSION_TIME_OUT",
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, WithMaskValueU32)]
pub enum StatusStartFlag {
    #[mask_value = 1]
    NoAvoid,
    NoDurationReduction,
    Loaded,
    NoRateReduction,
    NoIcon,
    #[mask_value = 16777216]
    ScriptProcDepth,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, WithMaskValueU32)]
pub enum JointBreak {
    #[mask_value = 1]
    Ankle, Wrist, Knee, Shoulder, Waist, Neck,
}

#[derive(Clone, Copy, Debug, WithMaskValueU32)]
pub enum RegenerationBlock {
    #[mask_value = 1]
    Hp, Sp,
}

#[derive(Clone, Copy, Debug, WithMaskValueU32)]
pub enum CloakingFlag {
    #[mask_value = 1]
    AdjacentWall,
    AllowAttacks,
    AllowSkills,
}

#[derive(Clone, Copy, Debug, WithMaskValueU32)]
pub enum StatusHealthFlag {
    #[mask_value = 1]
    Poison,
    Curse,
    Silence,
    Confusion,
    Blind,
    Angelus,
    Bleeding,
    DeadlyPoison,
}

#[derive(Clone, Copy, Debug, WithMaskValueU32)]
pub enum StatusDisplayFlag {
    #[mask_value = 1]
    Sight,
    Hiding,
    Cloaking,
    #[mask_value = 64]
    Invisible,
    #[mask_value = 16384]
    ChaseWalk,
    #[mask_value = 2048]
    Orcish,
    Wedding,
    Ruwach,
    #[mask_value = 65536]
    Christmas,
    #[mask_value = 262144]
    Summer,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusChangeRequest {
    pub kind: StatusChangeKind,
    pub duration_ms: i32,
    pub values: [i32; 4],
    pub rate: u16,
    pub flags: u32,
}

impl StatusChangeRequest {
    pub fn guaranteed(kind: StatusChangeKind, duration_ms: i32, value: i32) -> Self {
        Self { kind, duration_ms, values: [value, 0, 0, 0], rate: 10_000, flags: StatusStartFlag::NoAvoid.as_flag() }
    }

    pub fn has_flag(&self, flag: StatusStartFlag) -> bool { self.flags & flag.as_flag() != 0 }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusChange {
    pub kind: StatusChangeKind,
    pub values: [i32; 4],
    pub started_at: u128,
    pub expires_at: Option<u128>,
    pub next_periodic_at: u128,
    pub flags: u32,
    #[serde(default)]
    pub inherited_from: Option<(u32, u128)>,
}

impl StatusChangeKind {
    pub fn from_ailment(effect: StatusEffect) -> Option<Self> {
        match effect {
            StatusEffect::Stone => Some(Self::Stone), StatusEffect::Freeze => Some(Self::Freeze),
            StatusEffect::Stun => Some(Self::Stun), StatusEffect::Sleep => Some(Self::Sleep),
            StatusEffect::Poisoned => Some(Self::Poison), StatusEffect::Curse => Some(Self::Curse),
            StatusEffect::Silence => Some(Self::Silence), StatusEffect::Chaos | StatusEffect::Confuse => Some(Self::Confusion),
            StatusEffect::Blind => Some(Self::Blind), StatusEffect::Bleeding => Some(Self::Bleeding),
            StatusEffect::Coma => Some(Self::Coma), _ => None,
        }
    }
    pub fn ailment(self) -> Option<StatusEffect> {
        match self {
            Self::Stone | Self::StoneWait => Some(StatusEffect::Stone), Self::Freeze => Some(StatusEffect::Freeze),
            Self::Stun => Some(StatusEffect::Stun), Self::Sleep => Some(StatusEffect::Sleep),
            Self::Poison | Self::DeadlyPoison => Some(StatusEffect::Poisoned), Self::Curse => Some(StatusEffect::Curse),
            Self::Silence => Some(StatusEffect::Silence), Self::Confusion => Some(StatusEffect::Chaos),
            Self::Blind => Some(StatusEffect::Blind), Self::Bleeding => Some(StatusEffect::Bleeding), _ => None,
        }
    }

    pub fn blocks_movement(self) -> bool { self.metadata().states.get("NoMove").copied().unwrap_or(false) }
    pub fn blocks_attack(self) -> bool { self.metadata().states.get("NoAttack").copied().unwrap_or(false) }
    pub fn blocks_casting(self) -> bool { self.metadata().states.get("NoCast").copied().unwrap_or(false) }
    pub fn removed_by_damage(self) -> bool { self.metadata().flags.get("RemoveOnDamaged").copied().unwrap_or(false) }

    pub fn icon(self) -> Option<u16> {
        match self {
            Self::Sma => Some(crate::enums::client_effect_icon::ClientEffectIcon::SmaReady.value() as u16),
            Self::Provoke => Some(0), Self::Endure => Some(1), Self::TwoHandQuicken => Some(2),
            Self::Concentrate => Some(3), Self::Hiding => Some(4), Self::Cloaking => Some(5),
            Self::EnchantPoison => Some(6), Self::Quagmire => Some(8), Self::Angelus => Some(9),
            Self::Blessing => Some(10), Self::IncreaseAgi => Some(12), Self::DecreaseAgi => Some(13),
            Self::SlowPoison => Some(14), Self::Impositio => Some(15), Self::Suffragium => Some(16),
            Self::Aspersio => Some(17), Self::Benedictio => Some(18), Self::Kyrie => Some(19),
            Self::Magnificat => Some(20), Self::Gloria => Some(21), Self::LexAeterna => Some(22),
            Self::Adrenaline => Some(23), Self::WeaponPerfection => Some(24), Self::Overthrust => Some(25),
            Self::MaximizePower => Some(26), Self::TrickDead => Some(29), Self::Loud => Some(30),
            Self::EnergyCoat => Some(31), Self::Hallucination => Some(34),
            Self::AspdPotion0 => Some(37), Self::AspdPotion1 => Some(38), Self::AspdPotion2 => Some(39), Self::AspdPotion3 => Some(40),
            Self::SpeedUp0 => Some(41), Self::SpeedUp1 => Some(42), Self::Assumptio => Some(110), Self::WindWalk => Some(116),
            Self::Concentration => Some(105), Self::SteelBody => Some(87), Self::Bleeding => Some(124),
            Self::FireWeapon => Some(90), Self::WaterWeapon => Some(91), Self::WindWeapon => Some(92), Self::EarthWeapon => Some(93),
            Self::ShadowWeapon => Some(146), Self::GhostWeapon => Some(148), Self::ChangeUndead => Some(97),
            Self::AttackPotion => Some(150), Self::MagicAttackPotion => Some(151),
            Self::StrFood => Some(241), Self::AgiFood => Some(242), Self::VitFood => Some(243), Self::DexFood => Some(244),
            Self::IntFood => Some(245), Self::LukFood => Some(246),
            Self::CashStrFood => Some(271), Self::CashAgiFood => Some(272), Self::CashVitFood => Some(273),
            Self::CashDexFood => Some(274), Self::CashIntFood => Some(275), Self::CashLukFood => Some(276),
            _ => {
                self.metadata().icon.as_deref().and_then(|icon| crate::enums::client_effect_icon::ClientEffectIcon::try_from_string_ignore_case(icon.strip_prefix("EFST_").unwrap_or(icon)).ok()).map(|icon| icon.value() as u16)
            },
        }
    }
}

impl StatusChange {
    pub fn expired(&self, tick: u128) -> bool { self.expires_at.is_some_and(|until| tick >= until) }
    pub fn remaining_ms(&self, tick: u128) -> u32 {
        self.expires_at.map(|until| until.saturating_sub(tick).min(u32::MAX as u128) as u32).unwrap_or(u32::MAX)
    }

    pub fn bonuses(&self) -> Vec<BonusType> {
        use StatusChangeKind::*;
        let [value, second, third, fourth] = self.values;
        let stat = value.clamp(i8::MIN as i32, i8::MAX as i32) as i8;
        let amount = value.clamp(i16::MIN as i32, i16::MAX as i32) as i16;
        match self.kind {
            StrFood | CashStrFood | IncStr => vec![BonusType::Str(stat)],
            AgiFood | CashAgiFood | IncAgi => vec![BonusType::Agi(stat)],
            VitFood | CashVitFood | IncVit => vec![BonusType::Vit(stat)],
            IntFood | CashIntFood | IncInt => vec![BonusType::Int(stat)],
            DexFood | CashDexFood | IncDex => vec![BonusType::Dex(stat)],
            LukFood | CashLukFood | IncLuk => vec![BonusType::Luk(stat)],
            IncAllStatus => vec![BonusType::AllStats(stat)],
            Blessing if second != 0 => { let amount = second.clamp(-127, 127) as i8; vec![BonusType::Str(amount), BonusType::Int(amount), BonusType::Dex(amount)] },
            IncreaseAgi => vec![BonusType::Agi(second.clamp(-127, 127) as i8)],
            DecreaseAgi => vec![BonusType::Agi((-second).clamp(-127, 127) as i8)],
            Concentrate => vec![BonusType::Agi(third.clamp(-127, 127) as i8), BonusType::Dex(fourth.clamp(-127, 127) as i8)],
            Quagmire => vec![BonusType::Agi((-second).clamp(-127, 127) as i8), BonusType::Dex((-second).clamp(-127, 127) as i8)],
            SpearQuicken => vec![BonusType::Crit(3.0 * value as f32), BonusType::Flee((2 * value).clamp(-32768, 32767) as i16)],
            Sacrifice => {
                use crate::status_bonus::{AutoSpellFlag, BattleFlag, CombatProc, CombatProcKind, CombatTrigger};
                let mut proc = CombatProc::new(CombatTrigger::Attack, CombatProcKind::Spell, 10_000);
                proc.value = crate::enums::skill_enums::SkillEnum::PaSacrifice.id();
                proc.level = value.clamp(1, i16::MAX as i32) as i16;
                proc.flags = AutoSpellFlag::OtherTarget.as_flag();
                proc.battle_flags = BattleFlag::normalize(0, true);
                vec![BonusType::CombatProc(proc, 1)]
            }
            PoisonReact if second > 0 => {
                use crate::status_bonus::{AutoSpellFlag, BattleFlag, CombatProc, CombatProcKind, CombatTrigger};
                let mut proc = CombatProc::new(CombatTrigger::Hit, CombatProcKind::Spell, third.clamp(0, 100) * 100);
                proc.value = crate::enums::skill_enums::SkillEnum::TfPoison.id();
                proc.level = 5;
                proc.flags = AutoSpellFlag::OtherTarget.as_flag();
                proc.battle_flags = BattleFlag::Weapon.as_flag();
                vec![BonusType::CombatProc(proc, 1)]
            }
            Marionette | Marionette2 => {
                let sign = if self.kind == Marionette { -1 } else { 1 };
                let share = |packed: i32, shift: u32| (sign * ((packed >> shift) & 0xFF)) as i8;
                vec![
                    BonusType::Str(share(third, 0)), BonusType::Agi(share(third, 8)), BonusType::Vit(share(third, 16)),
                    BonusType::Int(share(fourth, 0)), BonusType::Dex(share(fourth, 8)), BonusType::Luk(share(fourth, 16)),
                ]
            }
            SunComfort => vec![BonusType::Def(second.clamp(0, i16::MAX as i32) as i16)],
            MoonComfort => vec![BonusType::Flee(second.clamp(0, i16::MAX as i32) as i16)],
            StarComfort => vec![BonusType::AspdPercentage(3.0 * value as f32)],
            Meltdown => vec![BonusType::BreakWeaponPercentage((second / 100).clamp(0, 127) as i8), BonusType::BreakArmorPercentage(third as f32 / 100.0)],
            Increasing => vec![BonusType::Agi(4), BonusType::Dex(4), BonusType::Hit(20)],
            MagicPower if fourth == 1 => vec![BonusType::MatkPercentage(second.clamp(-127, 127) as i8)],
            TrueSight => vec![BonusType::AllStats(5), BonusType::Crit(second as f32 / 10.0), BonusType::Hit(third.clamp(-32768, 32767) as i16), BonusType::AtkPercentage((2 * value).clamp(-127, 127) as i8)],
            Providence => vec![BonusType::ResistanceDamageFromElementPercentage(Element::Holy, second.clamp(-127, 127) as i8), BonusType::ResistanceDamageFromRacePercentage(MobRace::Demon, second.clamp(-127, 127) as i8)],
            Gloria => vec![BonusType::Luk(30)], Loud => vec![BonusType::Str(4)],
            Spurt => vec![BonusType::Str(10)],
            Fleet => vec![BonusType::AtkPercentage(third.clamp(-127, 127) as i8)],
            Bloodlust => vec![BonusType::AtkPercentage(second.clamp(-127, 127) as i8)],
            HomSpeed => vec![BonusType::Flee((10 + 10 * value).clamp(-32768, 32767) as i16)],
            HomChange => vec![BonusType::Vit(second.clamp(-127, 127) as i8), BonusType::Int(third.clamp(-127, 127) as i8)],
            HomDefence => if fourth == 1 { vec![BonusType::Def(second.clamp(-32768, 32767) as i16)] } else { vec![BonusType::Vit(second.clamp(-127, 127) as i8)] },
            Berserk => vec![BonusType::DisableHpRegen, BonusType::DisableSpRegen],
            Regeneration => {
                let mut bonuses = vec![];
                if fourth as u32 & RegenerationBlock::Hp.as_flag() != 0 { bonuses.push(BonusType::DisableHpRegen); }
                if fourth as u32 & RegenerationBlock::Sp.as_flag() != 0 { bonuses.push(BonusType::DisableSpRegen); }
                bonuses
            }
            Spirit if second == crate::enums::skill_enums::SkillEnum::SlHigh.id() as i32 => vec![BonusType::Str(((third / 65536) & 255) as i8), BonusType::Agi(((third / 256) & 255) as i8), BonusType::Vit((third & 255) as i8), BonusType::Int(((fourth / 65536) & 255) as i8), BonusType::Dex(((fourth / 256) & 255) as i8), BonusType::Luk((fourth & 255) as i8)],
            ChaseWalkStrength => vec![BonusType::Str(stat)],
            BattleOrders => vec![BonusType::Str(5), BonusType::Int(5), BonusType::Dex(5)],
            Leadership => vec![BonusType::Str(stat)],
            GloryWounds => vec![BonusType::Vit(stat)],
            SoulCold => vec![BonusType::Agi(stat)],
            HawkEyes => vec![BonusType::Dex(stat)],
            Powerup => vec![BonusType::AtkPercentage(stat)],
            Invincible => vec![BonusType::AtkPercentage(100)],
            Nen => vec![BonusType::Str(stat), BonusType::Int(stat)],
            AutoSpell => {
                use crate::status_bonus::{AutoSpellFlag, BattleFlag, CombatProc, CombatProcKind, CombatTrigger};
                let mut proc = CombatProc::new(CombatTrigger::Attack, CombatProcKind::Spell, fourth.saturating_mul(100));
                proc.value = second.max(0) as u32;
                proc.level = third.clamp(1, i16::MAX as i32) as i16;
                proc.flags = AutoSpellFlag::OtherTarget.as_flag() | AutoSpellFlag::SkillSelected.as_flag();
                proc.battle_flags = BattleFlag::normalize(0, true);
                vec![BonusType::CombatProc(proc, 1)]
            }
            Dancing => vec![BonusType::DisableSpRegen],
            Whistle => vec![BonusType::Flee(second.clamp(-32768, 32767) as i16), BonusType::PerfectDodge(third.clamp(-127, 127) as i8)],
            Humming => vec![BonusType::Hit(second.clamp(-32768, 32767) as i16)],
            PoemBragi => vec![BonusType::CastTimePercentage((-second).clamp(-127, 127) as i8), BonusType::AfterCastDelayPercentage((-third).clamp(-127, 127) as i8)],
            Fortune => vec![BonusType::Crit(second as f32 / 10.0)],
            Service4U => vec![BonusType::MaxspPercentage(second.clamp(-127, 127) as i8), BonusType::SpConsumption((-third).clamp(-127, 127) as i8)],
            AppleIdun => vec![BonusType::MaxhpPercentage(second.clamp(-127, 127) as i8)],
            DrumBattle => vec![BonusType::Atk(second.clamp(-32768, 32767) as i16), BonusType::Def(third.clamp(-32768, 32767) as i16)],
            Siegfried => [Element::Water, Element::Earth, Element::Fire, Element::Wind, Element::Poison, Element::Holy, Element::Dark, Element::Ghost, Element::Undead]
                .into_iter()
                .map(|element| BonusType::ResistanceDamageFromElementPercentage(element, second.clamp(-127, 127) as i8))
                .collect(),
            Volcano => vec![BonusType::Atk(second.clamp(-32768, 32767) as i16), BonusType::DamageUsingElementPercentage(Element::Fire, third.clamp(-127, 127) as i8)],
            ViolentGale => vec![BonusType::Flee(second.clamp(-32768, 32767) as i16), BonusType::DamageUsingElementPercentage(Element::Wind, third.clamp(-127, 127) as i8)],
            Deluge => vec![BonusType::MaxhpPercentage(second.clamp(-127, 127) as i8), BonusType::DamageUsingElementPercentage(Element::Water, third.clamp(-127, 127) as i8)],
            HitFood | IncHit => vec![BonusType::Hit(amount)], IncHitRate => vec![BonusType::HitPercentage(stat)],
            FleeFood | IncFlee => vec![BonusType::Flee(amount)],
            IncCritical => vec![BonusType::Crit(value as f32)],
            Endure => vec![BonusType::Mdef(amount), BonusType::EnableNoWalkDelay],
            Impositio => vec![BonusType::Atk((value.saturating_mul(5)).clamp(-32768, 32767) as i16)],
            AttackPotion | BaseAttackFood | WeaponAttackFood | ManuAttack | SplAttack => vec![BonusType::Atk(amount)],
            MagicAttackPotion | MagicAttackFood | ManuMagicAttack | SplMagicAttack => vec![BonusType::Matk(amount)],
            ManuDef | SplDef => vec![BonusType::Def(amount)],
            IncAttackRate => vec![BonusType::AtkPercentage(stat)],
            IncMagicAttackRate => vec![BonusType::MatkPercentage(stat)],
            IncDefRate | DefRate => vec![BonusType::DefPercentage(stat)],
            IncMaxHpRate => vec![BonusType::MaxhpPercentage(stat)], IncMaxSpRate => vec![BonusType::MaxspPercentage(stat)],
            IncreaseMaxHp => vec![BonusType::MaxhpPercentage(second.clamp(-127, 127) as i8)],
            IncreaseMaxSp => vec![BonusType::MaxspPercentage(second.clamp(-127, 127) as i8)],
            SpCostRate => vec![BonusType::SpConsumption(-stat)],
            Suffragium => vec![BonusType::CastTimePercentage((-15 * value).clamp(-127, 127) as i8)],
            SlowCast => vec![BonusType::CastTimePercentage((20 * value).clamp(-127, 127) as i8)],
            IncHealRate => vec![],
            CriticalWound => vec![],
            WeaponPerfection => vec![BonusType::EnableIgnoreSizeModifier],
            Intravision => vec![BonusType::EnableSeeHidden],
            Aspersio => vec![BonusType::ElementWeapon(Element::Holy)],
            FireWeapon => vec![BonusType::ElementWeapon(Element::Fire)], WaterWeapon => vec![BonusType::ElementWeapon(Element::Water)],
            WindWeapon => vec![BonusType::ElementWeapon(Element::Wind)], EarthWeapon => vec![BonusType::ElementWeapon(Element::Earth)],
            ShadowWeapon => vec![BonusType::ElementWeapon(Element::Dark)], GhostWeapon => vec![BonusType::ElementWeapon(Element::Ghost)],
            EnchantPoison => vec![BonusType::ElementWeapon(Element::Poison)],
            EnchantArms => Element::try_from_value(value as usize).map(|e| vec![BonusType::ElementWeapon(e)]).unwrap_or_default(),
            Benedictio => vec![BonusType::ElementDefense(Element::Holy)], ChangeUndead => vec![BonusType::ElementDefense(Element::Undead)],
            ArmorElementWater | ArmorElementFire | ArmorElementEarth | ArmorElementWind | ArmorResist => vec![
                BonusType::ResistanceDamageFromElementPercentage(Element::Water, stat),
                BonusType::ResistanceDamageFromElementPercentage(Element::Earth, second.clamp(-127, 127) as i8),
                BonusType::ResistanceDamageFromElementPercentage(Element::Fire, third.clamp(-127, 127) as i8),
                BonusType::ResistanceDamageFromElementPercentage(Element::Wind, fourth.clamp(-127, 127) as i8),
            ],
            CommonStatusResist | StatusResistance => vec![BonusType::ResistanceToStatusPercentage(StatusEffect::AllStatusEffect, value as f32)],
            AutoGuard => vec![],
            PartyFlee => vec![BonusType::Flee((value * 10).clamp(-32768, 32767) as i16)],
            ExplosionSpirits => vec![BonusType::Crit(second as f32 / 10.0)],
            MindBreaker => vec![BonusType::MatkPercentage((20 * value).clamp(0, 127) as i8)],
            Ske => vec![BonusType::AtkPercentage(100), BonusType::AtkPercentage(100), BonusType::AtkPercentage(100), BonusType::DefPercentage(-50)],
            MercFleeUp => vec![BonusType::Flee(second.clamp(-32768, 32767) as i16)],
            MercAttackUp => vec![BonusType::Atk(second.clamp(-32768, 32767) as i16)],
            MercHitUp => vec![BonusType::Hit(second.clamp(-32768, 32767) as i16)],
            MercHpUp => vec![BonusType::MaxhpPercentage(second.clamp(-127, 127) as i8)],
            MercSpUp => vec![BonusType::MaxspPercentage(second.clamp(-127, 127) as i8)],
            Poison | DeadlyPoison => vec![BonusType::DisableSpRegen], Bleeding => vec![BonusType::DisableHpRegen, BonusType::DisableSpRegen],
            MaximizePower => vec![BonusType::DisableSpRegen],
            NoRecovery => vec![BonusType::DisableHpRegen, BonusType::DisableSpRegen, BonusType::HpRegenFromItemPercentage(-100), BonusType::HpRegenFromSkillPercentage(-100)],
            _ => vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_preserve_classic_numeric_status_ids() {
        assert_eq!(StatusChangeKind::from_name("SC_Poison"), Some(StatusChangeKind::Poison));
        assert_eq!(StatusChangeKind::from_id(30), Some(StatusChangeKind::Blessing));
        assert_eq!(StatusChangeKind::MercQuicken.id(), 283);
        assert_eq!(StatusChangeKind::from_name("SC_ADORAMUS"), None);
    }
}
