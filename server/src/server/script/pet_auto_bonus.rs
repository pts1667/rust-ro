use std::sync::OnceLock;

use models::status::Status;
use models::status_bonus::AutoBonus;
use serde::Deserialize;

#[derive(Deserialize)]
struct PetBonusProgram {
    id: u32,
    class_id: u16,
    kind: String,
}

pub(crate) fn validate_program(program_id: u32, class_id: u16, kind: &str) -> Result<(), String> {
    static PROGRAMS: OnceLock<Vec<PetBonusProgram>> = OnceLock::new();
    let programs = PROGRAMS.get_or_init(|| {
        serde_json::from_str(include_str!("../../../../config/wasm/pet_bonus_programs.json"))
            .expect("Invalid compiled pet automatic bonus catalog")
    });
    if programs
        .iter()
        .any(|program| program.id == program_id && program.class_id == class_id && program.kind == kind)
    {
        Ok(())
    } else {
        Err(format!("Unknown compiled pet {kind} program {program_id} for class {class_id}"))
    }
}

pub(crate) fn source_is_active(status: &Status, definition: &AutoBonus) -> bool {
    status
        .script_context
        .as_ref()
        .and_then(|context| context.pet.as_ref())
        .is_some_and(|pet| pet.id == definition.source_pet_id && pet.intimacy > 0)
}
