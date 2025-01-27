use std::collections::VecDeque;

pub struct Player {
    pub balance: usize,
    pub name: String,
    pub bet: usize,
}

impl Player {
    pub fn name(&self) -> &str {
        &self.name
    }
}

pub struct Game {
    pub players: VecDeque<Player>,
    pub dealer_index: usize,
}

impl Game {
    pub fn new(player_count: usize, starting_balance: usize) -> Self {
        Self {
            players: (0..player_count)
                .map(|i| Player {
                    name: format!("Player{i}"),
                    balance: starting_balance,
                    bet: 0,
                })
                .collect(),
            dealer_index: 0,
        }
    }
    fn player_at_index(&self, index: usize) -> &Player {
        &self.players[index % self.players.len()]
    }
    pub fn dealer(&self) -> &Player {
        &self.players[self.dealer_index]
    }
    pub fn small(&self) -> &Player {
        self.player_at_index(self.dealer_index + 1)
    }
    pub fn big(&self) -> &Player {
        self.player_at_index(self.dealer_index + 2)
    }
    pub fn next_dealer(&mut self) {
        self.dealer_index = (self.dealer_index + 1) % self.players.len();
    }
}
