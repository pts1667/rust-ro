use std::sync::mpsc::SyncSender;

use configuration::configuration::SkillInTree;
use models::enums::EnumWithNumberValue;
use models::enums::class::JobName;
use models::enums::skill_enums::SkillEnum;
use models::status::KnownSkill;
use packets::packets::{Packet, PacketZcSkillinfoList, SKILLINFO};

use crate::server::model::events::client_notification::{CharNotification, Notification};
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::state::character::Character;
use crate::util::string::StringUtil;

pub struct SkillTreeService {
    client_notification_sender: SyncSender<Notification>,
    configuration_service: &'static GlobalConfigService,
}

impl SkillTreeService {
    pub fn new(client_notification_sender: SyncSender<Notification>, configuration_service: &'static GlobalConfigService) -> Self {
        Self {
            client_notification_sender,
            configuration_service,
        }
    }

    pub fn skill_tree(&self, character: &Character) -> Vec<KnownSkill> {
        let skilltree = self
            .configuration_service
            .get_job_skilltree(JobName::from_value(character.status.job as usize));
        let mut skills = vec![];

        let maybe_novice_basic = character.status.known_skills.iter().find(|s| s.value == SkillEnum::NvBasic);
        let mut platinium_novice_skills = character
            .status
            .known_skills
            .iter()
            .filter(|s| s.value.to_name().starts_with("NV") && s.value.is_platinium())
            .cloned()
            .collect::<Vec<KnownSkill>>();
        if maybe_novice_basic.is_none() {
            platinium_novice_skills.extend(vec![KnownSkill {
                value: SkillEnum::NvBasic,
                level: 0,
            }]);
            Self::append_script_grants(character, &mut platinium_novice_skills);
            return platinium_novice_skills;
        } else if maybe_novice_basic.unwrap().level < 9 {
            platinium_novice_skills.extend(vec![KnownSkill {
                value: SkillEnum::NvBasic,
                level: maybe_novice_basic.unwrap().level,
            }]);
            Self::append_script_grants(character, &mut platinium_novice_skills);
            return platinium_novice_skills;
        }
        Self::available_skills_in_tree(character, skilltree.tree(), &mut skills);
        for (_, parent_skills) in skilltree.parent_skills().iter() {
            Self::available_skills_in_tree(character, parent_skills, &mut skills);
        }
        Self::append_script_grants(character, &mut skills);
        skills
    }

    pub fn send_skill_tree(&self, character: &Character) {
        let skills = self.skill_tree(character);
        let mut range_source = models::status::StatusSnapshot::_from(&character.status);
        let mut effective_skills = character.status.known_skills.clone();
        for visible in &skills {
            if let Some(known) = effective_skills.iter_mut().find(|known| known.value == visible.value) {
                known.level = known.level.max(visible.level);
            } else {
                effective_skills.push(*visible);
            }
        }
        range_source.set_known_skills(effective_skills);
        let mut equipment_grants = Self::equipment_grants(character);
        equipment_grants.extend(crate::server::service::script_character_service::taekwon_rank_skill_grants(
            &character.status,
        ));
        let skills_info: Vec<SKILLINFO> = skills
            .iter()
            .filter_map(|skill| {
                let skill_enum = skill.value;
                let know_level = skill.level;
                if let Some(skill) = skills::skill_enums::to_object(skill_enum, know_level) {
                    let mut skill_info = SKILLINFO::new(self.configuration_service.packetver());
                    skill_info.set_skid(skill.id() as i16);
                    skill_info.set_atype(skill.client_type() as i32);
                    skill_info.set_level(skill.level() as i16);

                    skill_info.set_spcost(skill.sp_cost() as i16);
                    let range = crate::server::script::skill::metadata::SkillMetadata::find(skill.id()).map_or_else(
                        || skill.range().unsigned_abs().min(14) as u16,
                        |metadata| metadata.player_range(&range_source, know_level),
                    );
                    skill_info.set_attack_range(range.min(i16::MAX as u16) as i16);
                    let mut skill_name: [char; 24] = [0 as char; 24];
                    skill_enum.to_name().fill_char_array(&mut skill_name);
                    skill_info.set_skill_name(skill_name);
                    let mut is_upgradable = 0_i8;
                    if !skill_enum.is_platinium()
                        && !character.game_systems.permanent_skill_grants.contains_key(&skill_enum.id())
                        && !character.status.script_skill_grants.contains_key(&skill_enum.id())
                        && !equipment_grants.iter().any(|grant| grant.value == skill_enum)
                    {
                        is_upgradable = if skill.level() < skill.max_level() { 1 } else { 0 };
                    }
                    skill_info.set_upgradable(is_upgradable);
                    return Some(skill_info);
                } else {
                    let metadata = crate::server::script::skill::metadata::SkillMetadata::find(skill_enum.id())?;
                    let config = self.configuration_service.find_skill_config(&(skill_enum.id() as i32).into())?;
                    let mut info = SKILLINFO::new(self.configuration_service.packetver());
                    info.set_skid(skill_enum.id() as i16);
                    info.set_atype(config.target_type().value() as i32);
                    info.set_level(know_level as i16);
                    let sp = metadata
                        .requires
                        .as_ref()
                        .and_then(|requires| requires.get("SpCost"))
                        .and_then(|cost| {
                            crate::server::script::skill::metadata::SkillMetadata::json_level_value(cost, know_level, "Amount")
                        })
                        .unwrap_or(0);
                    info.set_spcost(sp.clamp(0, i16::MAX as i32) as i16);
                    info.set_attack_range(metadata.player_range(&range_source, know_level).min(i16::MAX as u16) as i16);
                    let mut name = [0 as char; 24];
                    skill_enum.to_name().fill_char_array(&mut name);
                    info.set_skill_name(name);
                    info.set_upgradable(i8::from(
                        !skill_enum.is_platinium()
                            && know_level < metadata.max_level
                            && !character.game_systems.permanent_skill_grants.contains_key(&skill_enum.id())
                            && !character.status.script_skill_grants.contains_key(&skill_enum.id())
                            && !equipment_grants.iter().any(|grant| grant.value == skill_enum),
                    ));
                    return Some(info);
                }
            })
            .collect::<Vec<SKILLINFO>>();
        let mut packet_zc_skillinfo_list = PacketZcSkillinfoList::new(self.configuration_service.packetver());
        packet_zc_skillinfo_list.set_packet_length(
            (PacketZcSkillinfoList::base_len(self.configuration_service.packetver())
                + (skills_info.len() * SKILLINFO::base_len(self.configuration_service.packetver()))) as i16,
        );
        packet_zc_skillinfo_list.set_skill_list(skills_info);
        packet_zc_skillinfo_list.fill_raw();
        self.client_notification_sender
            .send(Notification::Char(CharNotification::new(
                character.char_id,
                packet_zc_skillinfo_list.raw,
            )))
            .unwrap_or_else(|_| error!("Failed to send notification packet_zc_skillinfo_list to client"));
    }

    fn available_skills_in_tree(character: &Character, skilltree: &Vec<SkillInTree>, skills: &mut Vec<KnownSkill>) {
        for skill_in_tree in skilltree.iter() {
            let skill = SkillEnum::from_name(skill_in_tree.name());
            let level = character
                .status
                .known_skills
                .iter()
                .find(|s| s.value.id() == skill.id())
                .map_or(0, |s| s.level);
            if let Some(requirements) = skill_in_tree.requires() {
                let fulfill_requirements = requirements.iter().all(|requirement| {
                    let requirement_skill = SkillEnum::from_name(requirement.name());
                    crate::server::service::script_character_service::learned_level(&character.status, requirement_skill.id())
                        >= requirement.level()
                });
                if fulfill_requirements {
                    skills.push(KnownSkill { value: skill, level })
                }
                continue;
            }
            if skill.is_platinium() || skill_in_tree.job_level() as u32 > character.get_job_level() {
                continue;
            }
            skills.push(KnownSkill { value: skill, level });
        }
    }

    fn append_script_grants(character: &Character, skills: &mut Vec<KnownSkill>) {
        for granted in crate::server::service::script_character_service::taekwon_rank_skill_grants(&character.status) {
            if let Some(existing) = skills.iter_mut().find(|skill| skill.value == granted.value) {
                existing.level = existing.level.max(granted.level);
            } else {
                skills.push(granted);
            }
        }
        for granted in character.status.known_skills.iter().filter(|skill| {
            skill.value.is_platinium()
                || character.game_systems.permanent_skill_grants.contains_key(&skill.value.id())
                || character.status.script_skill_grants.contains_key(&skill.value.id())
        }) {
            if let Some(existing) = skills.iter_mut().find(|skill| skill.value == granted.value) {
                existing.level = existing.level.max(granted.level);
            } else {
                skills.push(*granted);
            }
        }
        for granted in Self::equipment_grants(character) {
            if let Some(existing) = skills.iter_mut().find(|skill| skill.value == granted.value) {
                existing.level = existing.level.max(granted.level);
            } else {
                skills.push(granted);
            }
        }
        skills.sort_by_key(|skill| skill.value.id());
        skills.dedup_by_key(|skill| skill.value.id());
    }

    fn equipment_grants(character: &Character) -> Vec<KnownSkill> {
        crate::server::service::status_service::StatusService::instance()
            .to_snapshot(&character.status)
            .bonuses()
            .iter()
            .filter_map(|bonus| match bonus.bonus() {
                models::enums::bonus::BonusType::EnableSkillId(id, level) => {
                    SkillEnum::try_from_value(*id).ok().map(|value| KnownSkill { value, level: *level })
                }
                _ => None,
            })
            .collect()
    }
}
