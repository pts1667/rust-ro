use super::*;

pub(super) fn incubating_pet(
    systems: &TransactionalTree,
    record: &InventoryRecord,
) -> ConflictableTransactionResult<PetRecord, Error> {
    let pet_id = u32::from(record.card1 as u16) | (u32::from(record.card2 as u16) << 16);
    if record.card0 != 256 || pet_id == 0 || record.amount != 1 || record.equip != 0 {
        return abort("Invalid pet egg identity or quantity");
    }
    let pet: PetRecord = tx_required(systems, &key(b"pet/", pet_id))?;
    if pet.id != pet_id || !pet.incubating || pet.egg_item_id != record.item_id
        || (record.unique_id != 0 && record.unique_id != i64::from(pet_id))
    {
        return abort("An active or changed pet egg cannot be transferred");
    }
    if pet.owner_char_id != 0 {
        let owner: CharacterGameSystems = tx_read(systems, &character_key(pet.owner_char_id))?.unwrap_or_default();
        if owner.pet.as_ref().is_some_and(|active| active.id == pet_id) {
            return abort("The pet is still active with its owner");
        }
    }
    Ok(pet)
}

pub(super) fn transfer_egg(
    systems: &TransactionalTree,
    record: &InventoryRecord,
    destination_record_id: i32,
    destination_char_id: u32,
    source_char_id: u32,
) -> ConflictableTransactionResult<(), Error> {
    let mut pet = incubating_pet(systems, record)?;
    for char_id in [source_char_id, destination_char_id].into_iter().filter(|id| *id != 0) {
        let state: CharacterGameSystems = tx_read(systems, &character_key(char_id))?.unwrap_or_default();
        if state.pet.as_ref().is_some_and(|active| active.id == pet.id) {
            return abort("The pet is already active with a transfer participant");
        }
    }
    pet.owner_char_id = destination_char_id;
    pet.egg_inventory_id = destination_record_id;
    tx_write(systems, &key(b"pet/", pet.id), &pet)
}
