use script_sdk::{Function, Value};

use crate::server::model::events::game_event::ScriptSkillCast;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::state::server::ServerState;

pub(crate) fn unit_skill_request(configuration: &GlobalConfigService, source_id: u32, function: Function, args: &[Value]) -> Result<ScriptSkillCast, String> {
    let ground = function == Function::UnitSkillToPosition;
    let (skill_index, options) = match function {
        Function::UnitSkill if args.len() == 2 => (0, 2),
        Function::UnitSkillToId if (3..=8).contains(&args.len()) => (1, 4),
        Function::UnitSkillToPosition if (5..=9).contains(&args.len()) => (1, 5),
        _ => return Err("Invalid unit skill command arguments".into()),
    };
    let number = |index: usize| args.get(index).ok_or("Missing unit skill argument")?.number_value();
    let actor_id = |value: i32| u32::try_from(value).ok().filter(|id| *id > 0).ok_or("Invalid unit actor identifier".to_string());
    let source_id = if skill_index == 0 { source_id } else { actor_id(number(0)?)? };
    let target_id = if skill_index == 0 || ground { source_id } else { args.get(3).map(Value::number_value).transpose()?.map_or(Ok(source_id), actor_id)? };
    let skill = configuration.find_skill_config(&args[skill_index]).ok_or("Unknown unit skill")?;
    let level = u16::try_from(number(skill_index + 1)?).ok().filter(|level| *level > 0).ok_or("Invalid unit skill level")?;
    let ground = if ground { Some((u16::try_from(number(3)?).map_err(|_| "Invalid ground skill coordinate")?, u16::try_from(number(4)?).map_err(|_| "Invalid ground skill coordinate")?)) } else { None };
    let cast_time_adjust_ms = args.get(options).map(Value::number_value).transpose()?.unwrap_or(0).checked_mul(1000).ok_or("Unit skill cast time is out of bounds")?;
    let cast_cancel = args.get(options + 1).map(Value::number_value).transpose()?.map(|cancel| cancel > 0);
    let message_id = args.get(options + 2).map(Value::number_value).transpose()?.filter(|id| *id > 0).map(|id| u16::try_from(id).map_err(|_| "Invalid monster message identifier")).transpose()?;
    let ignore_range = args.get(options + 3).map(Value::number_value).transpose()?.unwrap_or(0) > 0;
    Ok(ScriptSkillCast { source_id, target_id, skill_id: skill.id, level, ground, cast_time_adjust_ms, cast_cancel, message_id, ignore_range })
}

pub(crate) fn normalize_unit_skill_actor_ids(state: &ServerState, request: &mut ScriptSkillCast) {
    let resolve = |id| state.characters().values().find(|character| character.account_id == id).map_or(id, |character| character.char_id);
    request.source_id = resolve(request.source_id);
    request.target_id = resolve(request.target_id);
}

#[cfg(test)]
mod tests {
    use super::*;
    use models::enums::skill_enums::SkillEnum;

    #[test]
    fn targeted_unit_skills_keep_actor_target_and_primary_cast_options() {
        crate::tests::common::before_all();
        let request = unit_skill_request(GlobalConfigService::instance(), 1, Function::UnitSkillToId,
            &[123.into(), "MG_COLDBOLT".into(), 2.into(), 456.into(), (-1).into(), 0.into(), 7.into(), 1.into()]).unwrap();
        assert_eq!((request.source_id, request.target_id, request.skill_id, request.level), (123, 456, SkillEnum::MgColdbolt.id(), 2));
        assert_eq!((request.ground, request.cast_time_adjust_ms, request.cast_cancel, request.message_id, request.ignore_range), (None, -1000, Some(false), Some(7), true));
        let request = unit_skill_request(GlobalConfigService::instance(), 1, Function::UnitSkillToId, &[123.into(), "SM_ENDURE".into(), 1.into()]).unwrap();
        assert_eq!(request.target_id, 123); assert_eq!(request.cast_cancel, None);
    }

    #[test]
    fn ground_unit_skills_keep_coordinates_and_reject_bad_or_overflowing_arguments() {
        crate::tests::common::before_all();
        let configuration = GlobalConfigService::instance();
        let args = vec![123.into(), "MG_SAFETYWALL".into(), 1.into(), 10.into(), 20.into(), 2.into(), 1.into(), 0.into(), 1.into()];
        let request = unit_skill_request(configuration, 1, Function::UnitSkillToPosition, &args).unwrap();
        assert_eq!((request.ground, request.cast_time_adjust_ms, request.cast_cancel, request.message_id, request.ignore_range), (Some((10, 20)), 2000, Some(true), None, true));
        let mut args = args; args[3] = (-1).into();
        assert!(unit_skill_request(configuration, 1, Function::UnitSkillToPosition, &args).is_err());
        args[3] = 10.into(); args[5] = i32::MAX.into();
        assert!(unit_skill_request(configuration, 1, Function::UnitSkillToPosition, &args).is_err());
    }
}
