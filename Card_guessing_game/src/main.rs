use rand::Rng;
use std::{thread, time::Duration};
use anyhow::{Context, Result};
use serde::{Serialize, Deserialize};
use std::io::{self, Read};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct MyConfig {
    min_card: u8,
    max_card: u8,
    player_num: u8,
    agent_turn_delay: u64
}

/// Player struct that contains player ID, 
/// their card, whether they are a player, etc
struct Player {
    id: u8,
    card: u8,
    is_player: bool, 
}

/// Game struct that contains the card pool,
/// amount of players, some game parameters and  
/// a list of all players (both agents and human)
struct Game {
    pool: Vec<u8>,
    player_num: u8,
    players: Vec<Player>,
    agent_turn_delay: u64,
    player_pos_guess: u8,
    player_card_guess: u8,
    player_actual_card: u8,
    player_actual_pos: u8,
}

fn load_config() -> io::Result<MyConfig> {
    // Read the configuration file
    let config_contents = fs::read_to_string("config.json")?;

    // Parse the JSON content into the MyConfig struct
    let config: MyConfig = serde_json::from_str(&config_contents)
        .context("Could not parse config json");

    // You can now use `config` in your application
    println!("{:?}", config);

    Ok(config)
}
fn main() {
    let config = load_config();
    new_game();

}

/// Create and initialise a new game 
fn new_game() {
    println!("\n");
    println!("-------- NEW GAME ----------");
    
    // Attempt to create a new game, print context
    // on failure, then terminate game creation. Else, 
    // unpack the Game object and continue
    let game_res = create_game(2, 14, 8, 1, 2000);
    let mut game = match game_res {
        Ok(val)  => {
            Some(val)
        } 
        Err(val) => {
            println!("Something went wrong initialising the game. Context: \'{}\'", val);
            None
        }
    }.expect("Game creation aborted.");
       
    let result = game.game_loop(); 
    match result {
        Ok(_) => {
            new_game();
        },
        Err(val) => println!("Game loop entered abruptly: {}", val),
    };
}
/// This method is a factor for the Game object, it is returned
/// an anyhow result given the parameters:
/// min: minumum card number in pool 
/// max: maximum card number in pool
/// player_num: amount of players (including the player)
/// max_turns: [NOT IMPLEMENTED]
/// agent_turn_delay: artifical turn delay for agent in ms
fn create_game(min: u8, max: u8, player_num: u8, max_turns: u8, agent_turn_delay: u64) -> Result<Game> {    
    // We cannot have card (2 > card ID > 14),
    // return Error result if this is the case
    if max > 14 {
        return Err(anyhow::anyhow!("Init() failed because upper card limit set to: {}, ensure [max <= {}]",max, 14));
    }
    if min < 2 {
        return Err(anyhow::anyhow!("Init() failed because upper card limit set to: {}, ensure [min >= {}]",min, 2));
    }
    let nums: Vec<u8> = (min..=max).collect();
    println!(" Available card pool: {:?}", nums);

    let mut game_set = Game {
        pool: nums,
        player_num,
        players: Vec::<Player>::new(), 
        agent_turn_delay,
        player_pos_guess: 0,
        player_card_guess: 0,
        player_actual_card: 0,
        player_actual_pos: 0,
    };

    let mut players = Vec::new();
    //let mut is_player: bool;
    let total_players: u8 = player_num + 1;

    // Player order padding. For example, with 8 players,  
    // setting player order to 3 means that player can only get
    // turn 3, 4 or 5. 
    let player_order_pad = 3; 
    let min_index = player_order_pad; 
    let max_index: u8 = total_players-player_order_pad; 

    // Abort if this padding ruins order (i.e. padding greater than player count)       
    if max_index <= min_index {
        return Err(anyhow::anyhow!("Game settings could not be created because max_index <= min_index"));
    }
    
    // Obtain random player turn order
    let player_index = rand::thread_rng().gen_range(min_index..total_players);

    // Create all Player objects for this game including
    // one non-agent (human) player 
    for i in 0..total_players{
        let is_player = i == player_index;
        let index = rand::thread_rng().gen_range(0..game_set.pool.len());
        let drawn_element = game_set.pool.remove(index);
        //let card_name = get_card_name(drawn_element);
        players.push(Player{card: drawn_element, 
                            is_player, 
                            id: i, });
    }
    // Save players to the game settings and finish init
    game_set.players = players;
    game_set.player_num = game_set.players.len() as u8;
    Ok(game_set)
}


// Sleep thread for n milliseconds
fn sleep(duration: u64) -> Result<()> {
    thread::sleep(Duration::from_millis(duration));
    Ok(()) // TODO: thread::sleep doesn't implement Result, remove this..
}

/// All game-specific methods that can be called like
/// drawing a card, performing the player and AI turn, etc
impl Game {
    
    /// Core game loop, iteratively assigns turns to
    /// each agent and the player until every Player
    /// has performed its turn
    fn game_loop(&mut self) -> anyhow::Result<()> {
        // Prints the initial cards drawn by everyone
        // except for the player 
        self.print_cards()?;
        for i in 0..self.players.len() {
                let player = &self.players[i];
                
                // Obtain order position and card guess from either 
                // the player input or by the AI agent 
                let (position, card_guess) = if player.is_player {
                    let (pos, guess_id) = self.perform_player_turn(self.player_num, player.id)?;
                    self.player_pos_guess = pos;
                    self.player_card_guess = guess_id;
                    self.player_actual_card = player.card;
                    self.player_actual_pos = self.get_order_pos(player.id)?;
                    
                    (pos, guess_id)
                }
                else { 
                    // Obtain the position that the AI thinks it is in 
                    // along with its card, then artificially sleep 
                    let pos = self.perform_ai_turn(player.id)?;
                    let guess_id = self.players[player.id as usize].card;
                    // card_name = self.get_card_name(self.players[player.id as usize].card)?;
                    sleep(self.agent_turn_delay)?;
                    (pos, guess_id) 
                };
                
                let card_name = self.get_card_name(card_guess)?;
                println!("Player {} with card: [{}]: I think I am in place: {}", player.id, card_name, position);
            }
            // true if player guessed both position and
            // the card correctly, false in all other cases
            let correct: bool = self.player_pos_guess == self.player_actual_pos && self.player_card_guess == self.player_actual_card;
            let player_card = self.get_card_name(self.player_actual_card)?;
            match correct {
                true => println!("**** Congratulations, you guessed correctly. Your position is: {} and your card: {} ****", self.player_actual_pos, player_card),
                false => println!("**** Skill issue bro. You guessed card {} and position: {}, actual card: {} with position: {} ****", self.player_card_guess, self.player_pos_guess, self.player_actual_card, self.player_actual_pos),
            }
            println!("**** Want to play again? Of course you do, starting new round.. ****");
            sleep(2000)?;
            
            // Propagate OK to caller, new game is started 
            // from there if we got to this point (Ok)
            Ok(())
    
    }

    /// Iterates over all players, prints their cards
    /// but obfuscates the player card
    fn print_cards(&self) -> Result<()> {
        for i in 0..self.player_num {
            let player: &Player = &self.players[i as usize];
            
            // Obtain card name and print it for each agent
            // unless it is the human player.
            let card_name:String = if player.is_player {
                 "?".to_string()
            }
            else {
                 self.get_card_name(player.card)?
            };

            println!("Player: {} drew card: {}", player.id, card_name);
        }
        Ok(())
    }

    /// Accepts user input about card number and
    /// card position in relation to all agents, returns a
    /// (u8, u8) Result tuple
    fn perform_player_turn(&self, player_num: u8, player_id: u8) -> anyhow::Result<(u8, u8)> {
        let mut line = String::new();
        let mut line_2 : String = String::new();
        println!("Player {}, what is your card number (Ace=14, King=13, Queen=12, Jack=11, ..)", player_id);
        let _card_num_in = std::io::stdin().read_line(&mut line).unwrap();
        println!("And what is your global position between {} and {}", 1, player_num);
        let _card_num_in_2 = std::io::stdin().read_line(&mut line_2).unwrap();
        let trimmed_in = line.trim();
        let trimmed_in_2 = line_2.trim();
        let is_numeric = trimmed_in.parse::<i32>().is_ok() && trimmed_in_2.parse::<i32>().is_ok();
        
        // TODO include this in error handling (remove expect)
        if is_numeric {
            let card_numeric: u8 = trimmed_in.parse::<u8>()?;
            let pos_numeric: u8 = trimmed_in_2.parse::<u8>()?;
            Ok((pos_numeric, card_numeric))
        }
        else {
            // Not numeric, recurisvely prompt user again
            println!("I didn't understand your input, please make sure that both your position and card number are numeric.");
            self.perform_player_turn(player_num, player_id)
        }
    }
    
    /// Get the 'position' of a Player given their card. Position
    /// is ranked from high to low where having the position 1
    /// means that they think they have the highest card out of all players
    fn get_order_pos(&self, player_id: u8) -> anyhow::Result<u8> {
        let mut pos: u8 = 1;
        for element in &self.players {
            if element.card > self.players[player_id as usize].card {
                pos += 1;
            } 
        }    
    Ok(pos)
    }

    /// Perform AI turn. This returns the positiont the AI agent 
    /// thinks it is in.
    fn perform_ai_turn(&self, player_id: u8) -> anyhow::Result<u8> {
        let pos: u8 = self.get_order_pos(player_id)?; // TODO actually implement an AI element
        Ok(pos)
    }

    /// Given card ID, return its console-friendly name
    /// so that it can be printed to the consosole
    /// TODO consider moving console helpers to separate struct
    fn get_card_name(&self, card_id: u8) -> anyhow::Result<String> {
        let mut name: String;
        match card_id {
            11 => name = "11 (Boer)".to_string(),
            12 => name = "12 (Vrouw)".to_string(), 
            13 => name = "13 (Koning)".to_string(),
            14 => name = "14 (Aas)".to_string(),
            _ =>  name = card_id.to_string(),
        };
    Ok(name)
    }
}

