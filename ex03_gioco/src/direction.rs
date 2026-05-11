use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn random() -> Direction {
        match rand::thread_rng().gen_range(0..4) {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            _ => Direction::Right,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opposite() {
        assert_eq!(Direction::Up.opposite(), Direction::Down);
        assert_eq!(Direction::Down.opposite(), Direction::Up);
        assert_eq!(Direction::Left.opposite(), Direction::Right);
        assert_eq!(Direction::Right.opposite(), Direction::Left);
    }

    #[test]
    fn test_random() {
        let d = Direction::random();
        assert!(matches!(d, Direction::Up | Direction::Down | Direction::Left | Direction::Right));
    }
}
