use crate::server::Server;
use crate::server::model::map_flags::MapFlag;
use crate::server::model::map_item::MapItemType;
use crate::server::service::script_world_service::companion_status_snapshot;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

const EMPERIUM_MOB_ID: i16 = 1288;
const CASTLE_OWNER_FIELD: u8 = 1;

impl Server {
    pub(crate) fn player_combat_target_allowed(&self, state: &ServerState, source: &Character, target_id: u32) -> bool {
        self.player_ground_target_allowed(state, source, target_id, false)
    }

    pub(crate) fn player_ground_target_allowed(
        &self,
        state: &ServerState,
        source: &Character,
        target_id: u32,
        allow_dead_source: bool,
    ) -> bool {
        if (!allow_dead_source && (source.status.hp == 0 || source.is_dead())) || source.char_id == target_id {
            return false;
        }
        if companion_status_snapshot(source, target_id).is_some() {
            return false;
        }
        if let Some(unit) = state.ground_unit(target_id, source.current_map_name(), source.current_map_instance()) {
            return !unit.used;
        }
        let Some(item) = state.map_item(target_id, source.current_map_name(), source.current_map_instance()) else {
            return false;
        };
        if *item.object_type() == MapItemType::Mob {
            return state
                .get_map_instance_from_character(source)
                .and_then(|instance| {
                    instance
                        .state()
                        .get_mob(target_id)
                        .map(|mob| mob.status.hp() > 0 && (mob.mob_id != EMPERIUM_MOB_ID || self.emperium_attackable(state, source)))
                })
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
        if state.duels.same_duel(source.char_id, target.char_id) {
            return true;
        }
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
        if same_guild && guild_protected {
            return false;
        }
        !(flags.is_gvg() && self.guilds_allied(source.game_systems.guild_id, target.game_systems.guild_id))
    }

    pub(crate) fn guilds_allied(&self, first: u32, second: u32) -> bool {
        first != 0
            && second != 0
            && first != second
            && self
                .repository
                .guild(first)
                .ok()
                .flatten()
                .is_some_and(|guild| guild.allies.contains(&second))
    }

    fn emperium_attackable(&self, state: &ServerState, source: &Character) -> bool {
        let map = source.current_map_name();
        if !state.siege_active || !state.map_flags(&source.map_instance_key).enabled(MapFlag::GvgCastle) {
            return false;
        }
        let guild = source.game_systems.guild_id;
        if guild == 0 {
            return false;
        }
        let owner = self
            .repository
            .castle_value(map, CASTLE_OWNER_FIELD)
            .ok()
            .and_then(|owner| u32::try_from(owner).ok())
            .unwrap_or(0);
        owner != guild && !self.guilds_allied(guild, owner)
    }

    pub(crate) fn player_skill_requires_hostile_target(skill_id: u32) -> bool {
        crate::server::script::skill::metadata::SkillMetadata::find(skill_id)
            .is_some_and(|skill| skill.target_type.as_deref() == Some("Attack"))
    }
}
