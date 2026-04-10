use std::io;
use game_exercise::Game;

fn main() {
    println!("Enter n (board size): ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    println!("Enter m (number of food/poison): ");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let m: usize = input.trim().parse().unwrap();

    println!("Enter food amount: ");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let food_amount: u32 = input.trim().parse().unwrap();

    println!("Enter poison amount: ");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let poison_amount: u32 = input.trim().parse().unwrap();

    println!("Enter initial strength: ");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let initial_strength: u32 = input.trim().parse().unwrap();

    println!("Enter max moves: ");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let max_moves: u32 = input.trim().parse().unwrap();

    let mut game = Game::new(n, m, food_amount, poison_amount, initial_strength, max_moves);

    println!("Initial game state:");
    println!("{}", game);

    while game.step() {
        println!("{}", game);
        println!("Press enter to continue...");
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
    }

    if game.is_won() {
        println!("You win!");
    } else {
        println!("You lose!");
    }
}
