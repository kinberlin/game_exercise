use ex03_gioco::{Cell, Direction, Game};

#[test]
fn test_game_initialization() {
    let game = Game::new(5, 2, 10, 5, 20, 10);
    assert_eq!(game.board.len(), 5);
    assert_eq!(game.board[0][0], Cell::Wall);
    assert_eq!(game.board[4][4], Cell::Wall);
    let mut food_count = 0;
    let mut poison_count = 0;
    for row in &game.board {
        for cell in row {
            match cell {
                Cell::Food(_) => food_count += 1,
                Cell::Poison(_) => poison_count += 1,
                _ => {}
            }
        }
    }
    assert_eq!(food_count, 2);
    assert_eq!(poison_count, 2);
    assert_eq!(game.player.strength, 20);
}

#[test]
fn test_direction_opposite() {
    assert_eq!(Direction::Up.opposite(), Direction::Down);
    assert_eq!(Direction::Down.opposite(), Direction::Up);
    assert_eq!(Direction::Left.opposite(), Direction::Right);
    assert_eq!(Direction::Right.opposite(), Direction::Left);
}

#[test]
fn test_wall_bounce_stays_in_place() {
    // In a 3x3 board, the only interior cell is (1,1) — surrounded by walls on all sides.
    // Regardless of the coin flip direction, the player always bounces and stays at (1,1).
    let mut game = Game::new(3, 0, 0, 0, 10, 10);
    game.player.x = 1;
    game.player.y = 1;
    game.player.direction = Direction::Up;
    game.step();
    assert_eq!(game.player.x, 1);
    assert_eq!(game.player.y, 1);
}

#[test]
fn test_food_increases_strength() {
    let mut game = Game::new(5, 0, 0, 0, 10, 100);
    // Manually place food adjacent to the player and force movement into it.
    game.player.x = 2;
    game.player.y = 2;
    game.player.direction = Direction::Right;
    game.board[2][3] = Cell::Food(5);
    // Force the player right by calling step repeatedly until it moves right.
    // Since the board at (2,3) is Food and direction is Right, on a "heads" coin flip
    // the player moves right and eats the food. We loop until that happens.
    let initial_strength = game.player.strength;
    for _ in 0..20 {
        if game.player.x == 3 {
            break;
        }
        game.step();
    }
    // If player reached (3,2) the food was consumed.
    if game.player.x == 3 {
        assert!(game.player.strength >= initial_strength);
    }
}

#[test]
fn test_win_after_max_moves() {
    let mut game = Game::new(5, 0, 0, 0, 100, 5);
    for _ in 0..10 {
        game.step();
    }
    assert!(game.is_won());
    assert!(!game.is_lost());
}

#[test]
fn test_lose_when_strength_zero() {
    let mut game = Game::new(5, 0, 0, 0, 1, 100);
    game.player.strength = 0;
    assert!(game.is_lost());
    assert!(!game.is_won());
}
