use crate::direction::Direction;

#[derive(Clone, Debug)]
pub struct Player {
    pub x: usize,
    pub y: usize,
    pub direction: Direction,
    pub strength: u32,
}

impl Player {
    pub fn new(x: usize, y: usize, direction: Direction, strength: u32) -> Player {
        Player { x, y, direction, strength }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_creation() {
        let p = Player::new(2, 3, Direction::Up, 100);
        assert_eq!(p.x, 2);
        assert_eq!(p.y, 3);
        assert_eq!(p.direction, Direction::Up);
        assert_eq!(p.strength, 100);
    }

    #[test]
    fn test_player_clone() {
        let p = Player::new(1, 1, Direction::Left, 50);
        let q = p.clone();
        assert_eq!(p.x, q.x);
        assert_eq!(p.strength, q.strength);
    }
}
