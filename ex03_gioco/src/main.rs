use ex03_gioco::Game;
use std::io;

fn leggi_u32(prompt: &str) -> u32 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if let Ok(v) = input.trim().parse() {
            return v;
        }
        println!("Valore non valido, riprova.");
    }
}

fn leggi_usize(prompt: &str) -> usize {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if let Ok(v) = input.trim().parse::<usize>() {
            if v >= 3 { return v; }
        }
        println!("Valore non valido (minimo 3), riprova.");
    }
}

fn main() {
    let n = leggi_usize("Dimensione della matrice n (minimo 3):");
    let m = leggi_u32("Numero di celle cibo/veleno m:") as usize;
    let food_amount = leggi_u32("Quantità di forza per ogni cibo:");
    let poison_amount = leggi_u32("Quantità di danno per ogni veleno:");
    let initial_strength = leggi_u32("Forza iniziale del giocatore:");
    let max_moves = leggi_u32("Numero massimo di mosse:");

    let mut game = Game::new(n, m, food_amount, poison_amount, initial_strength, max_moves);

    println!("\nStato iniziale:");
    println!("{}", game);

    let mut input = String::new();
    while game.step() {
        println!("\n{}", game);
        println!("Premi invio per continuare...");
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
    }

    println!("\n{}", game);
    if game.is_won() {
        println!("\nHAI VINTO!");
    } else {
        println!("\nHAI PERSO!");
    }
}
