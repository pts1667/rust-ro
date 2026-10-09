use models::status_change::StatusChangeKind;

/// The unit a ground skill leaves on the map. Several skills can leave the same unit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GroundKind {
    WarpPortal,
    Firewall,
    Pneuma,
    Quagmire,
    Deluge,
    Volcano,
    ViolentGale,
    LandProtector,
    Thunderstorm,
    HeavenDrive,
    Meteor,
    StormGust,
    Vermilion,
    GrandCross,
    GrandDarkness,
    SkidTrap,
    AnkleSnare,
    LandMine,
    BlastMine,
    ClaymoreTrap,
    Shockwave,
    Flasher,
    Sandman,
    FreezingTrap,
    TalkieBox,
    Graffiti,
    ArrowShower,
    Earthquake,
    SafetyWall,
    Sanctuary,
    VenomDust,
    SpiderWeb,
    EvilLand,
    FirePillar,
    Demonstration,
    Basilica,
    FogWall,
    /// Warm of one star: 0 Sun, 1 Moon, 2 Star.
    Warm(u8),
}

impl GroundKind {
    pub fn view_id(self) -> u32 {
        match self {
            Self::WarpPortal => 129,
            Self::Firewall => 127,
            Self::Pneuma => 133,
            Self::SafetyWall => 126,
            Self::Sanctuary => 131,
            Self::VenomDust => 146,
            Self::SpiderWeb => 183,
            Self::Quagmire => 142,
            Self::Volcano => 154,
            Self::Deluge => 155,
            Self::ViolentGale => 156,
            Self::LandProtector => 157,
            Self::SkidTrap => 144,
            Self::AnkleSnare => 145,
            Self::LandMine => 147,
            Self::BlastMine => 143,
            Self::ClaymoreTrap => 152,
            Self::Shockwave => 148,
            Self::Flasher => 150,
            Self::Sandman => 149,
            Self::FreezingTrap => 151,
            Self::TalkieBox => 153,
            Self::Graffiti => 176,
            Self::Earthquake => 198,
            Self::FirePillar => 135,
            Self::Demonstration => 177,
            _ => 134,
        }
    }

    pub fn status(self) -> Option<StatusChangeKind> {
        match self {
            Self::Pneuma => Some(StatusChangeKind::Pneuma),
            Self::SafetyWall => Some(StatusChangeKind::SafetyWall),
            Self::Quagmire => Some(StatusChangeKind::Quagmire),
            Self::Deluge => Some(StatusChangeKind::Deluge),
            Self::Volcano => Some(StatusChangeKind::Volcano),
            Self::ViolentGale => Some(StatusChangeKind::ViolentGale),
            Self::Basilica => Some(StatusChangeKind::Basilica),
            Self::FogWall => Some(StatusChangeKind::FogWall),
            _ => None,
        }
    }

    /// Kinds that the actor (monster and NPC) cast pipeline can place and run to completion.
    pub fn actor_placeable(self) -> bool {
        matches!(
            self,
            Self::HeavenDrive
                | Self::Thunderstorm
                | Self::Pneuma
                | Self::SafetyWall
                | Self::Sanctuary
                | Self::VenomDust
                | Self::SpiderWeb
                | Self::EvilLand
                | Self::FirePillar
                | Self::Demonstration
                | Self::Quagmire
                | Self::Deluge
                | Self::Volcano
                | Self::ViolentGale
                | Self::LandProtector
                | Self::SkidTrap
                | Self::AnkleSnare
                | Self::LandMine
                | Self::BlastMine
                | Self::ClaymoreTrap
                | Self::Shockwave
                | Self::Flasher
                | Self::Sandman
                | Self::FreezingTrap
                | Self::ArrowShower
                | Self::Firewall
                | Self::Meteor
                | Self::StormGust
                | Self::Vermilion
                | Self::Earthquake
                | Self::GrandCross
                | Self::GrandDarkness
        )
    }

    /// Sage fields: one per caster, a new one replaces the previous.
    pub fn element_field(self) -> bool {
        matches!(self, Self::Deluge | Self::Volcano | Self::ViolentGale | Self::LandProtector)
    }

    pub fn damaging(self) -> bool {
        !matches!(
            self,
            Self::WarpPortal
                | Self::TalkieBox
                | Self::Graffiti
                | Self::Pneuma
                | Self::SafetyWall
                | Self::Sanctuary
                | Self::VenomDust
                | Self::SpiderWeb
                | Self::EvilLand
                | Self::GrandDarkness
                | Self::Quagmire
                | Self::Deluge
                | Self::Volcano
                | Self::ViolentGale
                | Self::LandProtector
                | Self::Basilica
                | Self::FogWall
                | Self::Warm(_)
        )
    }

    pub fn effect_range(self, configured: u16) -> u16 {
        match self {
            Self::Pneuma => 1,
            Self::Sanctuary | Self::VenomDust | Self::SpiderWeb | Self::FogWall => 0,
            _ => configured,
        }
    }

    /// Units that follow the trap placement rules and run through the trap-style tick.
    pub fn classic_unit(self) -> bool {
        self.trap() || matches!(self, Self::Graffiti | Self::FirePillar | Self::Demonstration)
    }

    /// Units that stay on the map after the actor that placed them is gone.
    pub fn outlives_source(self) -> bool {
        matches!(self, Self::AnkleSnare | Self::FirePillar | Self::Demonstration)
    }

    pub fn trap(self) -> bool {
        matches!(
            self,
            Self::SkidTrap
                | Self::AnkleSnare
                | Self::LandMine
                | Self::BlastMine
                | Self::ClaymoreTrap
                | Self::Shockwave
                | Self::Flasher
                | Self::Sandman
                | Self::FreezingTrap
                | Self::TalkieBox
        )
    }

    /// Units that take the text the caster typed when they are placed.
    pub fn carries_text(self) -> bool {
        matches!(self, Self::TalkieBox | Self::Graffiti)
    }
}

/// How a ground-target skill places itself when its unit alone does not say.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GroundPlacement {
    /// Ends the caster's own Basilica instead of raising a new one.
    Basilica,
    /// Opens the destination menu for the caster.
    WarpPortal,
    /// Erases the graffiti around the target point.
    Cleaner,
    /// Summons an alchemist creature at the target point.
    Summon,
    /// Applies the skill's area status around the target point.
    AreaStatus,
    /// At most this many units of the skill may be active from one caster.
    Limit(u8),
    /// The target cell must not already be covered by a unit of the skill.
    NoOverlap,
}
