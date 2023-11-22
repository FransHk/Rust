fn main() {
    
    println!("Hello, world!");
    let game_set = init(3, 14);
    
    debug_game(&game_set)
}

// Print game, console friendly
fn debug_game(game_set: &Game) {
   for element in &game_set.pool {
    //println!("{}", element);
    match element {
        11 => println!("Boer"), 
        12 => println!("Vrouw"), 
        13 => println!("Koning"), 
        14 => println!("Aas"), 
        _ =>  println!("{}", element.to_string()), 
   };
}
}

struct Game {
    pool: Vec<i8>,
}

// Return initialised game settings object
fn init(mut min: i8, mut max: i8) -> Game {
    if(max > 14){
        max = 14;
    }
    if(min < 2){
        min = 2;
    }
    
    let nums: Vec<i8> = (min..=max).collect();
    println!(" {:?}", nums);

    let game_set = Game {
        pool: nums,
    };
    return game_set   
}

