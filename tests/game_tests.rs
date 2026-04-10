use game_exercise::{Game, Direction, Cell};

#[test]
fn test_game_initialization() {
    let game = Game::new(5, 2, 10, 5, 20, 10);
    assert_eq!(game.board.len(), 5);
    assert_eq!(game.board[0][0], Cell::Wall);
    assert_eq!(game.board[4][4], Cell::Wall);
    // Check that there are 2 food and 2 poison
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
    assert!(game.player.strength == 20);
}

#[test]
fn test_direction_opposite() {
    assert_eq!(Direction::Up.opposite(), Direction::Down);
    assert_eq!(Direction::Down.opposite(), Direction::Up);
    assert_eq!(Direction::Left.opposite(), Direction::Right);
    assert_eq!(Direction::Right.opposite(), Direction::Left);
}

#[test]
fn test_wall_bounce() {
    let mut game = Game::new(3, 0, 0, 0, 10, 10);
    // Place player at edge
    game.player.x = 1;
    game.player.y = 1;
    game.player.direction = Direction::Up;
    // Move up to wall
    game.step();
    // Should bounce, direction down
    assert_eq!(game.player.direction, Direction::Down);
    assert_eq!(game.player.y, 1); // Stay in place
}