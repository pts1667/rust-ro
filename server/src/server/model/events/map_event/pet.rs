use super::{MapEventContext, MapEventHandler};

#[derive(Debug, PartialEq, Clone)]
pub struct PetCaptureClaimRequest {
    pub claim_id: u64,
    pub char_id: u32,
    pub target_id: u32,
    pub x: u16,
    pub y: u16,
    pub lure_item_id: i32,
    pub flag: u8,
    pub expires_at: u64,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PetCaptureFinalize {
    pub claim_id: u64,
    pub target_id: u32,
    pub commit: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PetLootClaimRequest {
    pub claim_id: u64,
    pub char_id: u32,
    pub pet_id: u32,
    pub target_id: u32,
    pub x: u16,
    pub y: u16,
    pub expires_at: u64,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PetLootFinalize {
    pub claim_id: u64,
    pub target_id: u32,
    pub commit: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PetLootDropRequest {
    pub claim_id: u64,
    pub char_id: u32,
    pub x: u16,
    pub y: u16,
    pub items: Vec<database::model::InventoryRecord>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PetLootDropFinalize {
    pub claim_id: u64,
    pub commit: bool,
}

impl MapEventHandler for PetCaptureClaimRequest {
    fn handle(self, ctx: &MapEventContext) {
        let request = self;
        ctx.service
            .claim_pet_capture(ctx.map_instance.state_mut().as_mut(), request, ctx.tick);
    }
}

impl MapEventHandler for PetCaptureFinalize {
    fn handle(self, ctx: &MapEventContext) {
        let request = self;
        ctx.service.finalize_pet_capture(ctx.map_instance.state_mut().as_mut(), request);
    }
}

impl MapEventHandler for PetLootClaimRequest {
    fn handle(self, ctx: &MapEventContext) {
        let request = self;
        ctx.service.claim_pet_loot(ctx.map_instance.state_mut().as_mut(), request, ctx.tick);
    }
}

impl MapEventHandler for PetLootFinalize {
    fn handle(self, ctx: &MapEventContext) {
        let request = self;
        ctx.service.finalize_pet_loot(ctx.map_instance.state_mut().as_mut(), request);
    }
}

impl MapEventHandler for PetLootDropRequest {
    fn handle(self, ctx: &MapEventContext) {
        let request = self;
        ctx.service.prepare_pet_loot_drop(ctx.map_instance.state_mut().as_mut(), request);
    }
}

impl MapEventHandler for PetLootDropFinalize {
    fn handle(self, ctx: &MapEventContext) {
        let request = self;
        ctx.service.finalize_pet_loot_drop(ctx.map_instance.state_mut().as_mut(), request);
    }
}
