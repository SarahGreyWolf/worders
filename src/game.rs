#![allow(dead_code)]

use rand::{thread_rng, Rng};

const SPACE: char = 0x20 as char;
const DEFAULT_BAG: [char; 100] = [
    SPACE, SPACE,  'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'B', 'B', 'C', 'C', 'D', 'D', 'D', 
    'D', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'F', 'F', 'G', 'G', 'G', 'H', 
    'H', 'I', 'I', 'I', 'I', 'I', 'I', 'I', 'I', 'I', 'J', 'K', 'L', 'L', 'L', 'L', 'M', 'M', 'N',
    'N', 'N', 'N', 'N', 'N', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'P', 'P', 'Q', 'R', 'R', 'R',
    'R', 'R', 'R', 'S', 'S', 'S', 'S', 'T', 'T', 'T', 'T', 'T', 'T', 'U', 'U', 'U', 'U', 'V', 'V',
    'W', 'W', 'X', 'Y', 'Y', 'Z'
];

pub struct GameState {
    bag: Vec<char>,
    players: [Option<Player>; 4],
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            bag: DEFAULT_BAG.to_vec(),
            players: [None, None, None, None],
        }
    }

    fn initialize(&mut self) {
        
    }

    pub fn add_player(&mut self, player: Player) -> bool {
        let mut empty_slots = self.players.iter_mut().enumerate().filter(|(_, s)| s.is_none());
        if let Some((first_slot, _)) = empty_slots.next() {
            self.players[first_slot] = Some(player);
            true
        } else {
            false
        }
    }

    fn draw(&mut self, player: &mut Player) -> bool {
        let index: usize = thread_rng().gen_range(0..self.bag.len()-1);
        if let Some(character) = self.bag.iter_mut().nth(index) {
            player.insert_into_hand(*character)
        } else {
            false
        }
    }
}

pub struct Player {
    username: &'static str,
    score: u32,
    hand: Vec<char>,
}

impl Player {
    pub fn new(username: &'static str) -> Self {
        Player {
            username,
            score: 0,
            hand: vec![],
        }
    }
    fn insert_into_hand(&mut self, character: char) -> bool {
        if self.hand.len() < 7 {
            self.hand.push(character);
            true
        } else {
            false
        }
    }
}

fn get_character_score(character: char) -> u32 {
    match character {
        'E' | 'A' | 'I' | 'O' | 'N' | 'R' | 'T' | 'L' | 'S' | 'U' => 1,
        'D' | 'G' => 2,
        'B' | 'C' | 'M' | 'P' => 3,
        'F' | 'H' | 'V' | 'W' | 'Y' => 4,
        'K' => 5,
        'J' | 'X' => 8,
        'Q' | 'Z' => 10,
        _ => 0
    }
}

fn calculate_score_from_placement(characters: Vec<char>) -> u32 {
    characters.iter().map(|c| get_character_score(*c) ).collect::<Vec<u32>>().iter().sum()
}
