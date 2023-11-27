use rand::Rng;

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
}

fn main() {
    println!("-------- DRAWING CARDS ----------");
    let game_set = init(2, 14, 3, 2); // Init initial game settings 
    // debug_game(&game_set); // Debug current game properties
    println!("-----------------------------");
    game_loop(&game_set); 
}

fn game_loop(game_set: &Game) {
   let mut pos: u8 = 0;
   for player in &game_set.players {
        if player.is_player {
            // do AI input
            pos = perform_player_turn(game_set, player.id);
        }
        else {
            // do player input 
            pos = perform_ai_turn(game_set, player.id);
        }
 
        let card_name = get_card_name(game_set.players[player.id as usize].card);
        println!("Player {} with card: [{}]: I think I am in place: {}", player.id, card_name, pos);
    }
}

// Get the position that a Player is in.
fn get_order_pos(game_set: &Game, player_id: u8) -> u8{
    let mut pos: u8 = 1;
    for element in &game_set.players {
        if element.card > game_set.players[player_id as usize].card {
            pos += 1;
        } 
    }    
    pos
}

// ALlow player to guess its own position
fn perform_player_turn(_game_set: &Game, player_id: u8) -> u8 {
    let mut line = String::new();
    println!("Player {}, in which place are you?", player_id);
    let player_in = std::io::stdin().read_line(&mut line).unwrap();
    let trimmed_in = line.trim();
    let is_numeric = trimmed_in.parse::<i32>().is_ok();
    if is_numeric {
        let pos_numeric: u8 = trimmed_in.parse::<u8>().expect("Failed to parse input");
        pos_numeric
    }
    else {
        return perform_player_turn(_game_set, player_id);
    }
}

// Return position that AI thinks it is in
fn perform_ai_turn(game_set: &Game, player_id: u8) -> u8 {
   let pos: u8 = get_order_pos(game_set, player_id);
   pos
}

// Given card ID, return its console-friendly name
fn get_card_name(card_id: u8) -> String {
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

// Accepts Game struct, updates its card pool and
// the active players in the game.
fn instantiate_players(mut game_set: Game, amount: u8) -> Game {
    let mut players = Vec::new();
    let mut is_player: bool;
    // Loop over player counts, have each draw a card 
    // and return the array of players
    for i in 0..amount+1{
        is_player = i == amount;
        let index = rand::thread_rng().gen_range(0..game_set.pool.len());
        let drawn_element = game_set.pool.remove(index);
        let card_name = get_card_name(drawn_element);
        players.push(Player{card: drawn_element, 
                            is_player, 
                            id: i, 
                            last_card_guess: 0, 
                            last_place_guess: 0});
        
        if !is_player{
            println!("Card drawn for agent: {} is: {}.", i, card_name);
        }
        else {
            println!("Card drawn for player: ???")
        }        

    }
    game_set.players = players;
    game_set
}

// Return initialised game settings object
fn init(mut min: u8, mut max: u8, player_num: u8, max_turns: u8) -> Game {
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
        players: Vec::<Player>::new(), // TODO figure out if this is OK?
    };

    game_set = instantiate_players(game_set, 2); // Update by adding players to game
    game_set   
}

