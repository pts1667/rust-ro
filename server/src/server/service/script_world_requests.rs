use super::{
    BookingRequest, CompanionRequest, FamilyRequest, GuildRequest, HomunculusRequest, PartyRequest, PetRequest, ScriptWorldService,
    StoreRequest, protocol,
};
use crate::server::Server;
use crate::server::model::battleground_queue::BattlegroundQueueCommand;
use crate::server::model::events::game_event::GameEvent;
use crate::server::model::game_systems::ItemContainer;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

#[derive(Debug, Clone, PartialEq)]
pub enum ScriptWorldRequest {
    Party(PartyRequest),
    Guild(GuildRequest),
    Battleground(BattlegroundRequest),
    Booking(BookingRequest),
    Family(FamilyRequest),
    Homunculus(HomunculusRequest),
    Companion(CompanionRequest),
    Pet(PetRequest),
    Store(StoreRequest),
    Container(ContainerRequest),
    FameList(u8),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BattlegroundRequest {
    BattlegroundMessage(String),
    BattlegroundQueue(crate::server::model::battleground_queue::BattlegroundQueueAction),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContainerRequest {
    StorageDeposit {
            index: u16,
            amount: u32,
        },
    StorageWithdraw {
            index: u16,
            amount: u32,
        },
    CloseStorage,
    ContainerTransfer {
            source: ItemContainer,
            destination: ItemContainer,
            index: u16,
            amount: u32,
        },
    SetCart(u8),
    ChangeCart(u8),
    RemoveOption,
}


impl ScriptWorldService {
    pub fn handle_request(
        &self,
        server: &Server,
        state: &mut ServerState,
        char_id: u32,
        request: ScriptWorldRequest,
        now: u64,
    ) -> Result<(), String> {
        if state.characters().get(&char_id).is_some_and(|character| character.game_systems.is_trading() || character.timing.skill_menu_blocked())
            && matches!(&request, ScriptWorldRequest::Container(ContainerRequest::ContainerTransfer { .. })
                | ScriptWorldRequest::Container(ContainerRequest::StorageDeposit { .. }) | ScriptWorldRequest::Container(ContainerRequest::StorageWithdraw { .. })
                | ScriptWorldRequest::Store(StoreRequest::PrepareVending { .. }) | ScriptWorldRequest::Store(StoreRequest::CreateVendingStore { .. })
                | ScriptWorldRequest::Store(StoreRequest::OpenVendingStore(_)) | ScriptWorldRequest::Store(StoreRequest::PurchaseVendingStore { .. })
                | ScriptWorldRequest::Store(StoreRequest::CreateBuyingStore { .. }) | ScriptWorldRequest::Store(StoreRequest::OpenBuyingStore(_))
                | ScriptWorldRequest::Store(StoreRequest::TradeBuyingStore { .. }) | ScriptWorldRequest::Pet(PetRequest::HatchPet(_))
                | ScriptWorldRequest::Pet(PetRequest::EquipPetAccessory(_)) | ScriptWorldRequest::Pet(PetRequest::PetMenu(1))
                | ScriptWorldRequest::Pet(PetRequest::CapturePet(_))) {
            return Err("Inventory operations are unavailable during trading".into());
        }
        let mut character = state.characters_mut().remove(&char_id).ok_or("Character is not online")?;
        let previous_pet_bonus = super::pet_bonus_state(&character);
        let skill = match &request {
            ScriptWorldRequest::Companion(CompanionRequest::UseCompanionSkill { skill_id, .. }) | ScriptWorldRequest::Companion(CompanionRequest::UseCompanionGroundSkill { skill_id, .. }) => {
                Some((*skill_id, None))
            }
            ScriptWorldRequest::Companion(CompanionRequest::FinishCompanionSkill(id)) => character
                .game_systems
                .companion_commands
                .get(id)
                .and_then(|command| command.cast.as_ref())
                .map(|cast| (cast.skill_id, Some(*id))),
            _ => None,
        };
        let result = self.request(server, state, &mut character, request, now);
        self.refresh_pet_bonuses(server, &mut character, previous_pet_bonus);
        if result.is_err() {
            if let Some((skill_id, actor_id)) = skill {
                if let Err(error) = self.send(
                    character.char_id,
                    protocol::skill_failed(skill_id, models::enums::skill::UseSkillFailure::Fail),
                ) {
                    warn!("Failed to send companion skill failure: {}", error);
                }
                if let Some(actor_id) = actor_id {
                    let mut packet = protocol::header(0x01B9);
                    packet.extend_from_slice(&actor_id.to_le_bytes());
                    if let Err(error) = self.area(&character, packet) {
                        warn!("Failed to cancel companion cast display: {}", error);
                    }
                }
            }
        }
        state.characters_mut().insert(char_id, character);
        result
    }

    fn request(
        &self,
        server: &Server,
        state: &mut ServerState,
        character: &mut Character,
        request: ScriptWorldRequest,
        now: u64,
    ) -> Result<(), String> {
        match request {
            ScriptWorldRequest::Party(request) => self.party_request(server, state, character, request),
            ScriptWorldRequest::Guild(request) => self.guild_request(server, state, character, request),
            ScriptWorldRequest::Battleground(request) => self.battleground_request(server, state, character, request),
            ScriptWorldRequest::Booking(request) => self.booking_request(state, character, request),
            ScriptWorldRequest::Family(request) => self.family_request(server, state, character, request),
            ScriptWorldRequest::Homunculus(request) => self.homunculus_request(server, state, character, request, now),
            ScriptWorldRequest::Companion(request) => self.companion_request(server, state, character, request, now),
            ScriptWorldRequest::Pet(request) => self.pet_request(server, state, character, request, now),
            ScriptWorldRequest::Store(request) => self.store_request(server, state, character, request, now),
            ScriptWorldRequest::Container(request) => self.container_request(server, state, character, request, now),
            ScriptWorldRequest::FameList(kind) => self.fame_list(character, kind),
        }
    }

    pub(crate) fn battleground_request(
        &self,
        server: &Server,
        state: &mut ServerState,
        character: &mut Character,
        request: BattlegroundRequest,
    ) -> Result<(), String> {
        match request {
            BattlegroundRequest::BattlegroundQueue(action) => {
                server.add_to_next_tick(GameEvent::BattlegroundQueue(BattlegroundQueueCommand { char_id: character.char_id, action }));
                Ok(())
            }
            BattlegroundRequest::BattlegroundMessage(message) => {
                if !message.starts_with(&format!("{} : ", character.name)) {
                    return Err("Battleground chat sender does not match the character".into());
                }
                server.battleground_chat(state, character, &message);
                Ok(())
            }
        }
    }

    pub(crate) fn container_request(
        &self,
        server: &Server,
        state: &mut ServerState,
        character: &mut Character,
        request: ContainerRequest,
        _now: u64,
    ) -> Result<(), String> {
        match request {
            ContainerRequest::SetCart(style) => self.set_cart(character, style, false),
            ContainerRequest::ChangeCart(style) => self.set_cart(character, style, true),
            ContainerRequest::RemoveOption => {
                self.remove_option(character)?;
                server.character_service().reload_client_side_status(character);
                Ok(())
            }
            ContainerRequest::ContainerTransfer {
                source,
                destination,
                index,
                amount,
            } => self.move_container(server, state, character, source, destination, index, amount),
            ContainerRequest::StorageDeposit { index, amount } => self.move_container(
                server,
                state,
                character,
                crate::server::model::game_systems::ItemContainer::Inventory,
                crate::server::model::game_systems::ItemContainer::Storage,
                index,
                amount,
            ),
            ContainerRequest::StorageWithdraw { index, amount } => self.move_container(
                server,
                state,
                character,
                crate::server::model::game_systems::ItemContainer::Storage,
                crate::server::model::game_systems::ItemContainer::Inventory,
                index,
                amount,
            ),
            ContainerRequest::CloseStorage => self.close_storage(character),
        
        }
    }
}
