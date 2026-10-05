use models::enums::cell::CellType;
use models::enums::vanish::VanishType;
use models::enums::{EnumWithMaskValueU16, EnumWithNumberValue};
use movement::position::Position;
use packets::packets::{Packet, PacketZcNotifyStandentry7, PacketZcNotifyVanish};
use script_sdk::Value;

use super::MapInstanceService;
use crate::server::model::events::game_event::GameEvent;
use crate::server::model::map_item::{MapItem, MapItemType};
use crate::server::script::skill::actor::NpcSkillState;
use crate::server::script::unit_data::{MapUnitDataRequest, NpcTransferPhase, NpcUnitDataField, ScriptNpcTransfer, UnitDataOperation};
use crate::server::state::map_instance::MapInstanceState;

pub(super) fn walkable(state: &MapInstanceState, x: u16, y: u16) -> bool {
    x < state.x_size()
        && y < state.y_size()
        && state
            .cells()
            .get(state.get_cell_index_of(x, y))
            .is_some_and(|cell| cell & CellType::Walkable.as_flag() != 0)
}

pub(super) fn warp_position(state: &MapInstanceState) -> Option<(u16, u16)> {
    let candidates = state
        .cells()
        .iter()
        .enumerate()
        .filter(|(_, cell)| **cell & CellType::Walkable.as_flag() != 0)
        .map(|(index, _)| {
            (
                (index % usize::from(state.x_size())) as u16,
                (index / usize::from(state.x_size())) as u16,
            )
        })
        .filter(|(x, y)| *x < state.x_size() && *y < state.y_size())
        .collect::<Vec<_>>();
    (!candidates.is_empty()).then(|| candidates[fastrand::usize(..candidates.len())])
}

pub(crate) fn npc_entry_packet(npc: &NpcSkillState, packetver: u32) -> Vec<u8> {
    use models::enums::look::LookType;
    let look = |kind: LookType| npc.looks[kind.value() as usize];
    let mut packet = PacketZcNotifyStandentry7::new(packetver);
    packet.set_packet_length(PacketZcNotifyStandentry7::base_len(packetver) as i16);
    packet.set_aid(npc.id);
    packet.set_gid(npc.id);
    packet.set_objecttype(MapItemType::Npc.value() as u8);
    packet.set_job(npc.sprite as i16);
    packet.set_pos_dir(
        Position {
            x: npc.x,
            y: npc.y,
            dir: npc.dir,
        }
        .to_pos(),
    );
    packet.set_speed(npc.snapshot().speed() as i16);
    packet.set_clevel(npc.level.min(i16::MAX as u32) as i16);
    packet.set_hp(npc.hp);
    packet.set_max_hp(npc.snapshot().max_hp());
    packet.set_sex(npc.sex);
    packet.set_head(look(LookType::Hair) as i16);
    packet.set_headpalette(look(LookType::HairColor));
    packet.set_body(look(LookType::Body) as i16);
    packet.set_bodypalette(look(LookType::ClothesColor));
    packet.set_accessory(look(LookType::HeadBottom));
    packet.set_accessory2(look(LookType::HeadTop));
    packet.set_accessory3(look(LookType::HeadMid));
    packet.set_weapon(u32::from(look(LookType::Weapon)));
    packet.set_shield(u32::from(look(LookType::Shield)));
    packet.set_robe(look(LookType::Robe));
    packet.set_state(npc.dead_sit);
    packet.fill_raw_with_packetver(Some(packetver));
    packet.raw
}

impl MapInstanceService {
    pub(super) fn vanish_npc(&self, state: &MapInstanceState, npc: &NpcSkillState) {
        let mut packet = PacketZcNotifyVanish::new(self.configuration_service.packetver());
        packet.set_gid(npc.id);
        packet.set_atype(VanishType::Teleport.value() as u8);
        packet.fill_raw();
        self.notify_area(state, npc.x, npc.y, packet.raw);
    }

    pub(super) fn refresh_npc(&self, state: &MapInstanceState, npc: &NpcSkillState) {
        self.notify_area(
            state,
            npc.x,
            npc.y,
            npc_entry_packet(npc, self.configuration_service.packetver()),
        );
    }

    pub fn unit_data(&self, state: &mut MapInstanceState, request: MapUnitDataRequest) {
        if !request.context.active() {
            request.context.reply(Err("Conversation is no longer active".into()));
            return;
        }
        let Some(npc) = state
            .script_skill_state
            .npcs
            .get(&request.actor_id)
            .filter(|npc| npc.script.scope_instance == request.actor_scope_instance)
            .cloned()
        else {
            request.context.reply(Ok((-1).into()));
            return;
        };
        let UnitDataOperation::Write { field, value, destination } = request.operation else {
            request.context.reply(Ok(npc.unit_data(request.map_id)));
            return;
        };
        if let Some(destination) = destination {
            if destination != *state.key() {
                state.script_skill_state.npcs.remove(&npc.id);
                state.script_skill_state.transferring_npcs.insert(npc.id);
                state.script_skill_state.casts.remove(&npc.id);
                state.script_skill_state.generations.remove(&npc.id);
                state.remove_item_with_id(npc.id);
                self.vanish_npc(state, &npc);
                self.server_task_queue
                    .add_to_first_index(GameEvent::ScriptNpcTransfer(ScriptNpcTransfer {
                        context: request.context,
                        original_position: (npc.x, npc.y),
                        npc,
                        origin: state.key().clone(),
                        destination,
                        error: None,
                        phase: NpcTransferPhase::Install,
                    }));
                return;
            }
        }
        let mut next = npc.clone();
        let result = match field {
            NpcUnitDataField::X | NpcUnitDataField::Y => (|| {
                let position = u16::try_from(value.number_value()?).map_err(|_| "NPC coordinate is outside the map")?;
                let (x, y) = if field == NpcUnitDataField::X {
                    (position, npc.y)
                } else {
                    (npc.x, position)
                };
                if !walkable(state, x, y) {
                    return Err("NPC destination cell is not walkable".into());
                }
                if next.hp == 0 {
                    next.initialize_for_cast();
                }
                next.x = x;
                next.y = y;
                Ok(())
            })(),
            NpcUnitDataField::Map => match warp_position(state) {
                Some((x, y)) => {
                    if next.hp == 0 {
                        next.initialize_for_cast();
                    }
                    next.x = x;
                    next.y = y;
                    Ok(())
                }
                None => Err("NPC destination map has no walkable cell".into()),
            },
            field => next.set_unit_data(field, &value),
        };
        if let Err(error) = result {
            request.context.reply(Err(error));
            return;
        }
        if (npc.x, npc.y) != (next.x, next.y) {
            self.vanish_npc(state, &npc);
            state.script_skill_state.casts.remove(&npc.id);
            state.script_skill_state.generations.remove(&npc.id);
        }
        state.insert_item(MapItem::new(next.id, next.sprite as i16, MapItemType::Npc));
        state.script_skill_state.npcs.insert(next.id, next.clone());
        self.refresh_npc(state, &next);
        request.context.reply(Ok(Value::default()));
    }

    pub fn install_script_npc(&self, state: &mut MapInstanceState, mut transfer: ScriptNpcTransfer) {
        let rollback = transfer.phase == NpcTransferPhase::Rollback;
        let result = (|| -> Result<(u16, u16), String> {
            if transfer.destination != *state.key() {
                return Err("NPC transfer reached the wrong map".into());
            }
            if !rollback && !transfer.context.active() {
                return Err("Conversation is no longer active".into());
            }
            if state.get_map_item(transfer.npc.id).is_some()
                || state.script_skill_state.npcs.contains_key(&transfer.npc.id)
                || !rollback && state.script_skill_state.transferring_npcs.contains(&transfer.npc.id)
            {
                return Err("NPC destination already contains this actor ID".into());
            }
            if rollback {
                Ok(transfer.original_position)
            } else {
                warp_position(state).ok_or_else(|| "NPC destination map has no walkable cell".into())
            }
        })();
        let (x, y) = match result {
            Ok(position) => position,
            Err(error) if !rollback => {
                transfer.error = Some(error);
                transfer.destination = transfer.origin.clone();
                transfer.phase = NpcTransferPhase::Rollback;
                self.server_task_queue.add_to_first_index(GameEvent::ScriptNpcTransfer(transfer));
                return;
            }
            Err(error) => {
                transfer.context.reply(Err(error));
                return;
            }
        };
        transfer.npc.x = x;
        transfer.npc.y = y;
        if !rollback && transfer.npc.hp == 0 {
            transfer.npc.initialize_for_cast();
        }
        state.insert_item(MapItem::new(transfer.npc.id, transfer.npc.sprite as i16, MapItemType::Npc));
        state.script_skill_state.npcs.insert(transfer.npc.id, transfer.npc.clone());
        self.refresh_npc(state, &transfer.npc);
        if rollback {
            state.script_skill_state.transferring_npcs.remove(&transfer.npc.id);
            transfer
                .context
                .reply(Err(transfer.error.unwrap_or_else(|| "NPC transfer failed".into())));
        } else {
            transfer.context.reply(Ok(Value::default()));
            transfer.phase = NpcTransferPhase::Complete;
            self.server_task_queue.add_to_first_index(GameEvent::ScriptNpcTransfer(transfer));
        }
    }
}
