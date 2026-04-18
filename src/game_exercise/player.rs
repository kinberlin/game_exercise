use super::direction::Direction;

#[derive(Clone, Debug)]
pub struct Player {
    pub x: usize,
    pub y: usize,
    pub direction: Direction,
    pub strength: u32,
}

impl Player {
    pub fn new(x: usize, y: usize, direction: Direction, strength: u32) -> Self {
        Player {
            x,
            y,
            direction,
            strength,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_creation() {
        let player = Player::new(5, 5, Direction::Up, 100);
        assert_eq!(player.x, 5);
        assert_eq!(player.y, 5);
        assert_eq!(player.direction, Direction::Up);
        assert_eq!(player.strength, 100);
    }

    #[test]
    fn test_player_clone() {
        let player1 = Player::new(3, 4, Direction::Down, 50);
        let player2 = player1.clone();
        assert_eq!(player1.x, player2.x);
        assert_eq!(player1.strength, player2.strength);
    }
}
