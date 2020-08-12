use crate::common::PlayerId;
use crate::common::TeamAlliance;


pub struct PlayerList {
    player_count: u32,
    teams_count: u32,
    players: Vec<PlayerId>,
}

impl PlayerList {
    pub fn ffa(n_players: u32) -> Self {
        let mut players: Vec<PlayerId> = vec![];

        for id in 0..n_players {
            players.push(PlayerId::new(id, TeamAlliance::Alliance(id)));
        }

        PlayerList {
            player_count: n_players,
            teams_count: n_players,
            players: players,
        }
    }
}
