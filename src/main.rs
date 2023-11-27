use rand::Rng;
use std::{thread, time::Duration};

struct Player {
    id: u8,
    card: u8,
    is_player: bool, 
    last_place_guess: u8,
    last_card_guess: u8,
}

struct Game {
    pool: Vec<u8>,
    player_num: u8,
    max_turns: u8,
    players: Vec<Player>,
    agent_turn_delay: u64,
}

fn new_game() -> Game {
    println!("\n");
    println!("-------- NEW GAME ----------");
    let mut game = init(5, 14, 8, 1, 3000); // Init initial game settings 
    return game;
}

fn main() {
    let mut game_set: Game = new_game();
}

fn sleep(duration: u64)
{
    thread::sleep(Duration::from_millis(duration));
}

impl Game {

 // ALlow player to guess its own position
    fn perform_player_turn(self, player_num: u8, player_id: u8) -> (u8, u8) {
        let mut line = String::new();
        let mut line_2 : String = String::new();
        println!("Player {}, what is your card number (Ace=14, King=13, Queen=12, Jack=11, ..)", player_id);
        let _card_num_in = std::io::stdin().read_line(&mut line).unwrap();
        println!("And what is your global position between {} and {}", 1, player_num);
        let _card_num_in_2 = std::io::stdin().read_line(&mut line_2).unwrap();
        
        let trimmed_in = line.trim();
        let trimmed_in_2 = line_2.trim();

        let is_numeric = trimmed_in.parse::<i32>().is_ok() && trimmed_in_2.parse::<i32>().is_ok();
        if is_numeric {
            let card_numeric: u8 = trimmed_in.parse::<u8>().expect("Failed to parse input");
            let pos_numeric: u8 = trimmed_in_2.parse::<u8>().expect("Failed to parse input");
            (pos_numeric, card_numeric)
        }
        else {
            // Not numeric, recurisvely prompt user again
            self.perform_player_turn(player_num, player_id)
        }
    }

   fn game_loop(self) {
        let mut pos: u8 = 0;
        let mut card: u8 = 0;

        let mut player_card_guess: u8 = 0;
        let mut player_pos_guess: u8 = 0;
        let mut actual_pos: u8 = 0;
        let mut actual_card: u8 = 0;
        let mut player_card: String = "".to_string();

        for player in &self.players {
                let card_name: String;

                if player.is_player {
                    // do AI input
                    (pos, card) = self.perform_player_turn(self.player_num, player.id);
                    player_pos_guess = pos;
                    player_card_guess = card;
                    
                    actual_card = player.card;
                    actual_pos = self.get_order_pos(player.id);
                    player_card = self.get_card_name(player.card);
                
                    card_name = "?".to_string();
                }
                else {
                    // do player input 
                    pos = self.perform_ai_turn(player.id);
                    card_name = self.get_card_name(self.players[player.id as usize].card);
                    sleep(self.agent_turn_delay);
                }
        
                println!("Player {} with card: [{}]: I think I am in place: {}", player.id, card_name, pos);

            }
            let correct: bool = player_pos_guess == actual_pos && player_card_guess == actual_card;
            match correct {
                true => println!("**** Congratulations, you guessed correctly. Your position is: {} and your card: {} ****", actual_pos, player_card),
                false => println!("**** You lost. You guessed card {} and position: {}, actual card: {} with position: {}", player_card_guess, player_pos_guess, actual_card, actual_pos),
            }
            
            sleep(2000);
            new_game();
        // println!("Game concluded, player guessed pos: {}, actual pos: {} for card: {}", player_pos_guess, actual_pos, player_card);
    
}

    // Get the position that a Player is in.
    fn get_order_pos(self, player_id: u8) -> u8{
        let mut pos: u8 = 1;
        for element in &self.players {
            if element.card > self.players[player_id as usize].card {
                pos += 1;
            } 
        }    
        pos
    }

    // Return position that AI thinks it is in
    fn perform_ai_turn(self, player_id: u8) -> u8 {
        let pos: u8 = self.get_order_pos(player_id);
        pos
    }

    // Given card ID, return its console-friendly name
    fn get_card_name(self, card_id: u8) -> String {
        let mut name: String;
        match card_id {
            11 => name = "Boer".to_string(),
            12 => name = "Vrouw".to_string(), 
            13 => name = "Koning".to_string(),
            14 => name = "Aas".to_string(),
            _ =>  name = card_id.to_string(),
        };
        name
    }
}

// Return initialised game settings object
fn init(mut min: u8, mut max: u8, player_num: u8, max_turns: u8, agent_turn_delay: u64) -> Game {
    if max > 14 {
        max = 14;
    }
    if min < 2 {
        min = 2;
    }
    
    let nums: Vec<u8> = (min..=max).collect();
    println!(" Available card pool: {:?}", nums);

    let mut game_set = Game {
        pool: nums,
        player_num,
        max_turns,
        players: Vec::<Player>::new(), 
        agent_turn_delay,
    };

    let mut players = Vec::new();
    let mut is_player: bool;
    let total_players: u8 = player_num + 1;
    // Loop over player counts, have each draw a card 
    // and return the array of players
    let player_order_pad = 3;
    let min_index = player_order_pad; // Minimum turn order
    let max_index: u8 = total_players-player_order_pad; // Max turn order (i.e. player can never be last)
        
    if(max_index <= min_index) {
        panic!("Too few players to enforce order padding of: {}", player_order_pad);
    }

    let player_index = rand::thread_rng().gen_range(min_index..total_players);
    for i in 0..total_players{
        is_player = i == player_index;
        let index = rand::thread_rng().gen_range(0..game_set.pool.len());
        let drawn_element = game_set.pool.remove(index);
        //let card_name = get_card_name(drawn_element);
        players.push(Player{card: drawn_element, 
                            is_player, 
                            id: i, 
                            last_card_guess: 0, 
                            last_place_guess: 0});
    }
    game_set.players = players;
    game_set.player_num = game_set.players.len() as u8;


    game_set
}

// Print game, console friendly
fn debug_game(game_set: &Game) {
   println!("---------------------------------");    
   println!("Debugging game with {} players and {} max turns", &game_set.player_num, &game_set.max_turns);
   println!("Cards still in the current game pool (not drawn): {:?}", game_set.pool);
   
   for element in &game_set.pool {
    //println!("{}", element);
    match element {
        11 => println!("Boer"), 
        12 => println!("Vrouw"), 
        13 => println!("Koning"), 
        14 => println!("Aas"), 
        _ =>  println!("{}", element.to_string()), 
   }};

   for element in &game_set.players {
    println!("Player: {} has card {}, is human player: {}", element.id, element.card, element.is_player);
   }
}


