use crate::common::PId;
use crate::common::PlayerId;
use crate::common::TeamAlliance;
use std::collections::HashMap;

pub struct PlayerList {
    player_count: PId,
    teams_count: PId,
    players: HashMap<PId, PlayerId>, // Hashmap because it is easy to query from PId.
}

impl PlayerList {
    pub fn ffa(n_players: PId) -> Self {
        let mut players: HashMap<PId, PlayerId> = HashMap::new();

        for id in 0..n_players {
            let player = PlayerId::new(id, TeamAlliance::Alliance(id));
            players.insert(id, player);
        }

        PlayerList {
            player_count: n_players,
            teams_count: n_players,
            players: players,
        }
    }

    pub fn get(&self, id: PId) -> Option<&PlayerId> {
        self.players.get(&id)
    }
}
