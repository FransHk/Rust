use rand::Rng;

fn main() {
    
    println!("Hello, world!");
    let game_set = init(3, 14, 3, 2);
    let (players, game) = instantiate_players(game_set, 2);
    debug_game(&game);
}

// Print game, console friendly
fn debug_game(game_set: &Game) {
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


}


fn instantiate_players(mut game_set: Game, amount: i8) -> (Vec<Player>, Game){
    let mut players = Vec::new();
    let mut bot: bool = false; 

    // Loop over player counts, have each draw a card 
    // and return the array of players
    for i in 0..amount {
        let index = rand::thread_rng().gen_range(0..game_set.pool.len());
        let drawn_element = game_set.pool.remove(index);

        players.push(Player{card: drawn_element, is_bot: bot});
        println!("Card drawn for player {}: {}, is_bot: {}", i, drawn_element, bot);

        bot = true; // ensure all but first player are set to be bots
    }

    println!("Pool after drawing cards: {:?}", game_set.pool);
    (players, game_set)
}

fn game_loop(game_set: &Game) {

}

struct Player {
    card: i8,
    is_bot: bool,
}

struct Game {
    pool: Vec<i8>,
    player_num: i8,
    max_turns: i8,
    //players: Vec<Player>,
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

    let game_set = Game {
        pool: nums,
        player_num: player_num,
        max_turns: max_turns
    };
    return game_set   
}

