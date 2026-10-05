#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CombatActorKind {
    #[default]
    Player,
    Monster,
    Npc,
    Homunculus,
    Mercenary,
    Pet,
    SkillUnit,
}
