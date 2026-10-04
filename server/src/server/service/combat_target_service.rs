use crate::server::Server;
use crate::server::model::map_flags::MapFlag;
use crate::server::model::map_item::MapItemType;
use crate::server::service::script_world_service::companion_status_snapshot;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

impl Server {
    pub(crate) fn player_combat_target_allowed(&self, state: &ServerState, source: &Character, target_id: u32) -> bool {
        if source.status.hp == 0 || source.is_dead() || source.char_id == target_id {
            return false;
        }
        if companion_status_snapshot(source, target_id).is_some() {
            return false;
        }
        let Some(item) = state.map_item(target_id, source.current_map_name(), source.current_map_instance()) else {
            return false;
        };
        if *item.object_type() == MapItemType::Mob {
            return state
                .get_map_instance_from_character(source)
                .and_then(|instance| instance.state().get_mob(target_id).map(|mob| mob.status.hp() > 0))
                .unwrap_or(false);
        }
        if *item.object_type() == MapItemType::Pet {
            return false;
        }
        let target = state
            .characters()
            .get(&target_id)
            .or_else(|| state.companion_owner(target_id, source.current_map_name(), source.current_map_instance()));
        let Some(target) = target.filter(|target| {
            target.map_instance_key == source.map_instance_key
                && target.char_id != source.char_id
                && target.status.hp > 0
                && !target.is_dead()
        }) else {
            return false;
        };
        if target_id != target.char_id && !companion_status_snapshot(target, target_id).is_some_and(|snapshot| snapshot.hp() > 0) {
            return false;
        }
        let flags = state.map_flags(&source.map_instance_key);
        if !flags.versus(state.siege_active) {
            return false;
        }
        let same_party = source.game_systems.party_id > 0 && source.game_systems.party_id == target.game_systems.party_id;
        let party_protected =
            !(flags.enabled(MapFlag::Pvp) && flags.enabled(MapFlag::PvpNoParty)) && !(flags.is_gvg() && flags.enabled(MapFlag::GvgNoParty));
        if same_party && party_protected {
            return false;
        }
        let same_guild = source.game_systems.guild_id > 0 && source.game_systems.guild_id == target.game_systems.guild_id;
        let guild_protected = !(flags.enabled(MapFlag::Pvp) && flags.enabled(MapFlag::PvpNoGuild));
        !(same_guild && guild_protected)
    }

    pub(crate) fn player_skill_requires_hostile_target(skill_id: u32) -> bool {
        crate::server::script::skill::metadata::SkillMetadata::find(skill_id)
            .is_some_and(|skill| skill.target_type.as_deref() == Some("Attack"))
    }
}
