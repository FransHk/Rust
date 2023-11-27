use rand::Rng;

struct Player {
    id: i8,
    card: i8,
    is_bot: bool, 
    last_place_guess: i8,
    last_card_guess: i8,
}

struct Game {
    pool: Vec<i8>,
    player_num: i8,
    max_turns: i8,
    players: Vec<Player>,
}


fn main() {
    let game_set = init(2, 14, 3, 2); // Init initial game settings 
    debug_game(&game_set); // Debug current game properties
    game_loop(&game_set); 
}

fn game_loop(game_set: &Game) {
   for player in &game_set.players {
        println!("Turn of player number: {}", player.id);
        if player.is_bot {
            // do AI input
            perform_ai_turn(&game_set, player.id);
        }
        else {
        perform_player_turn(&game_set, player.id);
        }
    }
}



fn perform_player_turn(game_set: &Game, player_id: i8) -> (){
    let mut line = String::new();
    println!("Player {}, in which place are you?", player.id);
    let player_in = std::io::stdin().read_line(&mut line).unwrap();
    let trimmed_in = line.trim();
    let is_numeric = trimmed_in.parse::<i32>().is_ok();
    
    if(is_numeric) {
       print("Player {}: I think I am in place: {}", player_id, pos); 
    }

}

fn perform_ai_turn(game_set: &Game, player_id: i8) -> (){
    //println!("Performing AI turn for player {}", ai_player.id);
    let mut pos: u8 = 1;
    for element in &game_set.players {
        if(element.card > game_set.players[0].card) {
            pos += 1;
        }
    }    

    println!("Player {}: I think I am in place: {}", player_id, pos);

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
    println!("Player: {} has card {}", element.id, element.card);
   }
}

// Accepts Game struct, updates its card pool and
// the active players in the game.
fn instantiate_players(mut game_set: Game, amount: i8) -> Game {
    let mut players = Vec::new();
    let mut bot: bool = false; 

    // Loop over player counts, have each draw a card 
    // and return the array of players
    for i in 0..amount {
        let index = rand::thread_rng().gen_range(0..game_set.pool.len());
        let drawn_element = game_set.pool.remove(index);

        players.push(Player{card: drawn_element, is_bot: bot, id: i, last_card_guess: 0, last_place_guess: 0});
        println!("Card drawn for player {}: {}, is_bot: {}", i, drawn_element, bot);

        bot = true; // ensure all but first player are set to be bots
    }
    game_set.players = players;
    game_set
}

// Return initialised game settings object
fn init(mut min: i8, mut max: i8, player_num: i8, max_turns: i8) -> Game {
    if max > 14 {
        max = 14;
    }
    if min < 2 {
        min = 2;
    }
    
    let nums: Vec<i8> = (min..=max).collect();
    println!(" {:?}", nums);

    let mut game_set = Game {
        pool: nums,
        player_num: player_num,
        max_turns: max_turns,
        players: Vec::<Player>::new(), // TODO figure out if this is OK?
    };

    game_set = instantiate_players(game_set, 2); // Update by adding players to game
    return game_set   
}

