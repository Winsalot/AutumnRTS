//use std::collections::HashMap;

// Possible teams for every entity to belong to.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TeamAlliance{
	Neutral,
	Alliance(u32),
	Spectator,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PlayerId{
	id: u32, 
	team: TeamAlliance
}

impl PlayerId {
	pub fn new(id: u32, team: TeamAlliance) -> Self {
		PlayerId{
			id: id,
			team: team,
		}
	}

	pub fn get_id(&self) -> u32{
		self.id
	}

	pub fn get_team(&self) -> TeamAlliance {
		self.team
	}
}

// Holds information of every player's team.
pub struct PlayerList{
	player_count: u32, 
	teams_count: u32,
	players: Vec<PlayerId>
}

impl PlayerList {

	pub fn ffa(n_players: u32) -> Self{

		let mut players: Vec<PlayerId> = vec![];

		for id in 0..n_players {
			players.push(PlayerId::new(id, TeamAlliance::Alliance(id)));
		}

		PlayerList{
			player_count: n_players, 
			teams_count: n_players,
			players: players,
		}
	}
}
