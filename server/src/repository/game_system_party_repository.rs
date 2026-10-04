use super::*;
use crate::server::model::game_systems::{PartyChange, PartyRecord};

fn name_key(name: &str) -> Vec<u8> {
    [b"party_name/".as_slice(), name.to_lowercase().as_bytes()].concat()
}

fn record_key(id: u32) -> Vec<u8> {
    key(b"party/", id)
}

fn next_revision(party: &mut PartyRecord) -> ConflictableTransactionResult<(), Error> {
    party.revision = party
        .revision
        .checked_add(1)
        .ok_or_else(|| sled::transaction::ConflictableTransactionError::Abort(Error::new("Party revision overflow".into())))?;
    Ok(())
}

fn install_members(
    systems: &TransactionalTree,
    characters: &TransactionalTree,
    before: Option<PartyRecord>,
    party: Option<PartyRecord>,
    removed_members: Vec<u32>,
) -> ConflictableTransactionResult<PartyChange, Error> {
    let mut affected = BTreeSet::new();
    if let Some(before) = &before {
        affected.extend(before.members.iter().copied());
    }
    if let Some(party) = &party {
        affected.extend(party.members.iter().copied());
    }
    if affected.len() > 12 || party.as_ref().is_some_and(|party| party.members.len() > 12) {
        return abort("Party exceeds its member limit");
    }
    let mut member_states = BTreeMap::new();
    let mut member_records = Vec::new();
    for id in affected {
        member_records.push(tx_required(characters, &id.to_be_bytes())?);
        let mut state: CharacterGameSystems = tx_read(systems, &character_key(id))?.unwrap_or_default();
        let expected_id = if before.as_ref().is_some_and(|before| before.members.contains(&id)) {
            before.as_ref().unwrap().id
        } else {
            0
        };
        if state.party_id != expected_id {
            return abort("Party membership changed during the operation");
        }
        if let Some(party) = party.as_ref().filter(|party| party.members.contains(&id)) {
            state.party_id = party.id;
            state.party_name = party.name.clone();
            state.party_leader_id = party.leader_char_id;
            state.party_members = party.members.clone();
        } else {
            state.party_id = 0;
            state.party_name.clear();
            state.party_leader_id = 0;
            state.party_members.clear();
        }
        write_state(systems, id, &mut state)?;
        member_states.insert(id, state);
    }
    if let Some(party) = &party {
        tx_write(systems, &record_key(party.id), party)?;
    } else if let Some(before) = &before {
        systems.remove(record_key(before.id))?;
        systems.remove(name_key(&before.name))?;
    }
    Ok(PartyChange {
        before,
        party,
        member_states,
        member_records,
        removed_members,
    })
}

fn authorized_party(systems: &TransactionalTree, actor: u32) -> ConflictableTransactionResult<PartyRecord, Error> {
    let state: CharacterGameSystems = tx_required(systems, &character_key(actor))?;
    let party: PartyRecord = tx_required(systems, &record_key(state.party_id))?;
    if !party.members.contains(&actor) {
        return abort("Character does not belong to this party");
    }
    Ok(party)
}

pub fn create(repository: &SledRepository, leader: u32, name: String, item_pickup: bool, item_share: bool) -> Result<PartyChange, Error> {
    let name = name.trim().to_string();
    if name.is_empty() || name.len() > 23 || name.chars().any(char::is_control) {
        return Err(Error::new("Invalid party name".into()));
    }
    Ok((
        &repository.database.game_systems,
        &repository.database.characters,
        &repository.database.metadata,
    )
        .transaction(|(systems, characters, metadata)| {
            let state: CharacterGameSystems = tx_read(systems, &character_key(leader))?.unwrap_or_default();
            if state.party_id != 0 {
                return abort("Character already belongs to a party");
            }
            if systems.get(name_key(&name))?.is_some() {
                return abort("Party name already exists");
            }
            let id = u32::try_from(next_id(metadata, b"party_id", 0)?)
                .map_err(|_| sled::transaction::ConflictableTransactionError::Abort(Error::new("Party identifier overflow".into())))?;
            let party = PartyRecord {
                id,
                revision: 1,
                name: name.clone(),
                leader_char_id: leader,
                members: vec![leader],
                exp_share: false,
                item_pickup,
                item_share,
            };
            let change = install_members(systems, characters, None, Some(party), Vec::new())?;
            tx_write(systems, &name_key(&name), &id)?;
            Ok(change)
        })?)
}

pub fn join(repository: &SledRepository, char_id: u32, party_id: u32, inviter: u32) -> Result<PartyChange, Error> {
    Ok(
        (&repository.database.game_systems, &repository.database.characters).transaction(|(systems, characters)| {
            let before: PartyRecord = tx_required(systems, &record_key(party_id))?;
            if before.leader_char_id != inviter || !before.members.contains(&inviter) {
                return abort("Party invitation is no longer authorized");
            }
            if before.members.len() >= 12 || before.members.contains(&char_id) {
                return abort("Party is full or already contains this character");
            }
            let joining: CharacterRecord = tx_required(characters, &char_id.to_be_bytes())?;
            for member in &before.members {
                let record: CharacterRecord = tx_required(characters, &member.to_be_bytes())?;
                if record.account_id == joining.account_id {
                    return abort("This account already has a character in the party");
                }
            }
            let mut party = before.clone();
            party.members.push(char_id);
            next_revision(&mut party)?;
            install_members(systems, characters, Some(before), Some(party), Vec::new())
        })?,
    )
}

pub fn leave(repository: &SledRepository, member: u32, expeller: Option<u32>) -> Result<PartyChange, Error> {
    Ok(
        (&repository.database.game_systems, &repository.database.characters).transaction(|(systems, characters)| {
            let before = authorized_party(systems, member)?;
            if expeller.is_some_and(|actor| actor != before.leader_char_id) {
                return abort("Only the party leader can expel members");
            }
            if before.leader_char_id == member {
                let removed = before.members.clone();
                install_members(systems, characters, Some(before), None, removed)
            } else {
                let mut party = before.clone();
                party.members.retain(|id| *id != member);
                next_revision(&mut party)?;
                install_members(systems, characters, Some(before), Some(party), vec![member])
            }
        })?,
    )
}

pub fn leader(repository: &SledRepository, actor: u32, target: u32, expected_revision: u64) -> Result<PartyChange, Error> {
    Ok(
        (&repository.database.game_systems, &repository.database.characters).transaction(|(systems, characters)| {
            let before = authorized_party(systems, actor)?;
            if before.leader_char_id != actor || before.revision != expected_revision || !before.members.contains(&target) {
                return abort("Invalid or stale party leader change");
            }
            let mut party = before.clone();
            party.leader_char_id = target;
            next_revision(&mut party)?;
            install_members(systems, characters, Some(before), Some(party), Vec::new())
        })?,
    )
}

pub fn options(
    repository: &SledRepository,
    actor: u32,
    expected_revision: u64,
    exp_share: bool,
    item_pickup: bool,
    item_share: bool,
) -> Result<PartyChange, Error> {
    Ok(
        (&repository.database.game_systems, &repository.database.characters).transaction(|(systems, characters)| {
            let before = authorized_party(systems, actor)?;
            if before.leader_char_id != actor || before.revision != expected_revision {
                return abort("Only the current party leader can change party options");
            }
            let mut party = before.clone();
            party.exp_share = exp_share;
            party.item_pickup = item_pickup;
            party.item_share = item_share;
            next_revision(&mut party)?;
            install_members(systems, characters, Some(before), Some(party), Vec::new())
        })?,
    )
}

pub fn records(repository: &SledRepository, party_id: u32) -> Result<Vec<CharacterRecord>, Error> {
    Ok(
        (&repository.database.game_systems, &repository.database.characters).transaction(|(systems, characters)| {
            let party: PartyRecord = tx_required(systems, &record_key(party_id))?;
            party.members.iter().map(|id| tx_required(characters, &id.to_be_bytes())).collect()
        })?,
    )
}

#[cfg(test)]
mod tests {
    use database::model::{AccountRecord, SeedData};

    use super::*;

    fn repository(count: u32) -> SledRepository {
        let repository = SledRepository::temporary().unwrap();
        let accounts = (0..count)
            .map(|index| AccountRecord {
                account_id: 2_000_000 + index,
                username: format!("party-account-{index}"),
                password: "password".into(),
            })
            .collect();
        let characters = (0..count)
            .map(|index| CharacterRecord {
                char_id: (150_000 + index) as i32,
                account_id: (2_000_000 + index) as i32,
                name: format!("Member{index}"),
                base_level: 50,
                inventory_slots: 100,
                last_map: "prontera".into(),
                ..CharacterRecord::default()
            })
            .collect();
        repository
            .database
            .seed(
                &SeedData {
                    accounts,
                    characters,
                    ..SeedData::default()
                },
                false,
            )
            .unwrap();
        repository
    }

    #[test]
    fn party_roster_leadership_departures_and_name_reuse_commit_for_every_member() {
        let repository = repository(4);
        let mut source = CharacterGameSystems::default();
        source.font = 7;
        source.partner_id = 150_003;
        let source = repository.save_character_game_systems(150_000, &source).unwrap();
        let created = repository.create_party(150_000, "Classic Party".into(), true, true).unwrap();
        let party = created.party.unwrap();
        assert_eq!(created.member_states[&150_000].revision, source.revision + 1);
        assert_eq!(created.member_states[&150_000].font, 7);
        let joined = repository.join_party(150_001, party.id, 150_000).unwrap();
        let party = joined.party.unwrap();
        for member in [150_000, 150_001] {
            let state = repository.character_game_systems(member).unwrap();
            assert_eq!(state.party_members, vec![150_000, 150_001]);
            assert_eq!(state.party_name, "Classic Party");
            assert_eq!(state.party_leader_id, 150_000);
        }
        let before = repository.character_game_systems(150_000).unwrap();
        assert!(repository.join_party(150_002, party.id, 150_001).is_err());
        assert!(repository.change_party_options(150_001, party.revision, true, true, true).is_err());
        assert!(repository.leave_party(150_000, Some(150_001)).is_err());
        assert!(repository.create_party(150_002, " classic party ".into(), false, false).is_err());
        assert_eq!(repository.character_game_systems(150_000).unwrap(), before);
        let leadership = repository.change_party_leader(150_000, 150_001, party.revision).unwrap();
        let party = leadership.party.unwrap();
        assert!(repository.join_party(150_002, party.id, 150_000).is_err());
        let options = repository.change_party_options(150_001, party.revision, true, false, true).unwrap();
        let party = options.party.unwrap();
        assert!(party.exp_share && !party.item_pickup && party.item_share);
        assert_eq!(repository.character_game_systems(150_000).unwrap().party_leader_id, 150_001);
        let departed = repository.leave_party(150_000, None).unwrap();
        assert_eq!(departed.removed_members, vec![150_000]);
        assert_eq!(repository.character_game_systems(150_000).unwrap().party_id, 0);
        assert_eq!(repository.character_game_systems(150_001).unwrap().party_members, vec![150_001]);
        repository.join_party(150_002, party.id, 150_001).unwrap();
        let dissolved = repository.leave_party(150_001, None).unwrap();
        assert!(dissolved.party.is_none());
        assert_eq!(dissolved.removed_members, vec![150_001, 150_002]);
        for member in [150_001, 150_002] {
            let state = repository.character_game_systems(member).unwrap();
            assert_eq!((state.party_id, state.party_leader_id), (0, 0));
            assert!(state.party_members.is_empty() && state.party_name.is_empty());
        }
        assert!(repository.party(party.id).unwrap().is_none());
        assert_eq!(repository.character_game_systems(150_000).unwrap().partner_id, 150_003);
        assert!(repository.create_party(150_003, "Classic Party".into(), false, false).is_ok());
    }

    #[test]
    fn party_capacity_same_account_stale_revision_and_late_failure_roll_back_all_members() {
        let repository = repository(14);
        let created = repository
            .create_party(150_000, "Full Party".into(), false, false)
            .unwrap()
            .party
            .unwrap();
        for member in 150_001..150_012 {
            repository.join_party(member, created.id, 150_000).unwrap();
        }
        let full = repository.party(created.id).unwrap().unwrap();
        let before = repository.character_game_systems(150_000).unwrap();
        assert!(repository.join_party(150_012, created.id, 150_000).is_err());
        assert!(repository.change_party_leader(150_000, 150_001, created.revision).is_err());
        assert_eq!(repository.character_game_systems(150_000).unwrap(), before);
        let cleared = repository.leave_party(150_011, None).unwrap().party.unwrap();
        repository
            .database
            .characters
            .transaction(|characters| {
                let mut record: CharacterRecord = tx_required(characters, &150_012_u32.to_be_bytes())?;
                record.account_id = 2_000_001;
                tx_write(characters, &150_012_u32.to_be_bytes(), &record)
            })
            .unwrap();
        assert!(repository.join_party(150_012, full.id, 150_000).is_err());
        let before: Vec<_> = cleared
            .members
            .iter()
            .map(|id| (*id, repository.character_game_systems(*id).unwrap()))
            .collect();
        repository
            .database
            .characters
            .transaction(|characters| {
                characters.remove(150_010_u32.to_be_bytes().as_slice())?;
                Ok::<_, sled::transaction::ConflictableTransactionError<Error>>(())
            })
            .unwrap();
        assert!(
            repository
                .change_party_options(150_000, cleared.revision, true, true, true)
                .is_err()
        );
        assert_eq!(repository.party(full.id).unwrap().unwrap(), cleared);
        for (id, state) in before {
            assert_eq!(repository.character_game_systems(id).unwrap(), state);
        }
    }
}
