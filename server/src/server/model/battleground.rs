use std::collections::HashMap;

pub const MAX_BG_MEMBERS: usize = 30;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BgPoint {
    pub map: String,
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BgMember {
    pub char_id: u32,
    pub last_x: u16,
    pub last_y: u16,
    pub last_hp: u32,
    pub seen_on_battleground_map: bool,
    pub entry_point: Option<BgPoint>,
}

#[derive(Debug, Clone, Default)]
pub struct BgTeam {
    pub id: u32,
    pub members: Vec<BgMember>,
    pub cemetery: Option<BgPoint>,
    pub quit_event: String,
    pub die_event: String,
    pub active_event: String,
    pub deserter_seconds: u32,
}

#[derive(Debug, Default)]
pub struct Battlegrounds {
    teams: HashMap<u32, BgTeam>,
    by_char: HashMap<u32, u32>,
    scores: HashMap<String, (u16, u16)>,
    pub queues: crate::server::model::battleground_queue::BgQueues,
}

impl Battlegrounds {
    pub fn create(&mut self, cemetery: Option<BgPoint>, quit_event: String, die_event: String, active_event: String, deserter_seconds: u32) -> u32 {
        let id = (1..).find(|id| !self.teams.contains_key(id)).unwrap_or(1);
        self.teams.insert(id, BgTeam { id, members: vec![], cemetery, quit_event, die_event, active_event, deserter_seconds });
        id
    }

    pub fn members_mut(&mut self) -> impl Iterator<Item = (u32, &mut BgMember)> {
        self.teams.iter_mut().flat_map(|(id, team)| team.members.iter_mut().map(move |member| (*id, member)))
    }

    pub fn score(&self, map: &str) -> (u16, u16) {
        self.scores.get(map).copied().unwrap_or_default()
    }

    pub fn set_score(&mut self, map: &str, first: u16, second: u16) {
        self.scores.insert(map.to_string(), (first, second));
    }

    pub fn team(&self, id: u32) -> Option<&BgTeam> {
        self.teams.get(&id)
    }

    pub fn team_mut(&mut self, id: u32) -> Option<&mut BgTeam> {
        self.teams.get_mut(&id)
    }

    pub fn teams(&self) -> impl Iterator<Item = &BgTeam> {
        self.teams.values()
    }

    pub fn team_of(&self, char_id: u32) -> u32 {
        self.by_char.get(&char_id).copied().unwrap_or(0)
    }

    pub fn join(&mut self, bg_id: u32, char_id: u32, position: (u16, u16), entry_point: Option<BgPoint>) -> bool {
        if self.by_char.contains_key(&char_id) {
            return false;
        }
        let Some(team) = self.teams.get_mut(&bg_id) else { return false };
        if team.members.len() >= MAX_BG_MEMBERS {
            return false;
        }
        team.members.push(BgMember {
            char_id,
            last_x: position.0,
            last_y: position.1,
            last_hp: u32::MAX,
            seen_on_battleground_map: false,
            entry_point,
        });
        self.by_char.insert(char_id, bg_id);
        true
    }

    /// Removes the member and returns the team id, the member's entry point and the team's deserter time.
    pub fn leave(&mut self, char_id: u32) -> Option<(u32, Option<BgPoint>, u32)> {
        let bg_id = self.by_char.remove(&char_id)?;
        let team = self.teams.get_mut(&bg_id)?;
        let index = team.members.iter().position(|member| member.char_id == char_id)?;
        let member = team.members.remove(index);
        Some((bg_id, member.entry_point, team.deserter_seconds))
    }

    pub fn delete(&mut self, bg_id: u32) -> Vec<u32> {
        let Some(team) = self.teams.remove(&bg_id) else { return vec![] };
        let members: Vec<u32> = team.members.iter().map(|member| member.char_id).collect();
        for char_id in &members {
            self.by_char.remove(char_id);
        }
        members
    }

    pub fn member_ids(&self, bg_id: u32) -> Vec<u32> {
        self.teams.get(&bg_id).map(|team| team.members.iter().map(|member| member.char_id).collect()).unwrap_or_default()
    }

    pub fn prune(&mut self, is_online: impl Fn(u32) -> bool) -> Vec<u32> {
        let offline: Vec<u32> = self.by_char.keys().copied().filter(|id| !is_online(*id)).collect();
        for char_id in &offline {
            self.leave(*char_id);
        }
        offline
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn teams_reuse_the_lowest_free_id_and_members_belong_to_one_team() {
        let mut bg = Battlegrounds::default();
        let first = bg.create(None, String::new(), String::new(), String::new(), 0);
        let second = bg.create(None, String::new(), String::new(), String::new(), 0);
        assert_eq!((first, second), (1, 2));
        assert!(bg.join(first, 10, (1, 1), None));
        assert!(!bg.join(second, 10, (1, 1), None));
        assert_eq!(bg.team_of(10), first);
        assert_eq!(bg.delete(first), vec![10]);
        assert_eq!(bg.team_of(10), 0);
        assert_eq!(bg.create(None, String::new(), String::new(), String::new(), 0), 1);
    }

    #[test]
    fn leave_returns_the_entry_point_and_prune_drops_offline_members() {
        let mut bg = Battlegrounds::default();
        let id = bg.create(None, String::new(), String::new(), String::new(), 0);
        let entry = BgPoint { map: "prontera".into(), x: 150, y: 150 };
        bg.join(id, 1, (0, 0), Some(entry.clone()));
        bg.join(id, 2, (0, 0), None);
        assert_eq!(bg.prune(|char_id| char_id == 1), vec![2]);
        assert_eq!(bg.leave(1), Some((id, Some(entry), 0)));
        assert_eq!(bg.leave(1), None);
        assert!(bg.member_ids(id).is_empty());
    }
}
