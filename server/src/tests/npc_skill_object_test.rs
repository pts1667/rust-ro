use models::enums::skill::SkillType;
use models::enums::skill_enums::SkillEnum;
use skills::npc;

use crate::server::service::mob_skill_ai::MobSkillEntry;

#[test]
fn monster_skill_objects_follow_damage_flags() {
    let attack = npc::to_object(SkillEnum::NpcPiercingatt, 1).unwrap();
    assert_eq!(attack.skill_type(), SkillType::Offensive);
    assert!(npc::to_object(SkillEnum::NpcPiercingatt, 1).is_some_and(|skill| skill.is_offensive_skill()));
    assert!(npc::to_object(SkillEnum::NpcPiercingatt, 11).is_none());

    let support = npc::to_object(SkillEnum::NpcEmotion, 1).unwrap();
    assert_eq!(support.skill_type(), SkillType::Support);
    assert!(!npc::to_object(SkillEnum::NpcEmotion, 1).is_some_and(|skill| skill.is_offensive_skill()));
}

#[test]
fn every_npc_skill_used_by_mob_db_has_an_object() {
    let entries: Vec<MobSkillEntry> = serde_json::from_str(include_str!("../../../config/mob_skills.json")).unwrap();
    for entry in entries {
        let Ok(skill) = SkillEnum::try_from_value(entry.skill_id) else { continue };
        if skill.to_name().starts_with("NPC_") {
            assert!(npc::to_object(skill, 1).is_some(), "{} has no monster skill object", skill.to_name());
        }
    }
}
