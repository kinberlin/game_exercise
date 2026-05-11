use std::fmt;
use rand::prelude::*;
use crate::cell::Cell;
use crate::player::Player;
use crate::direction::Direction;

#[derive(Clone, Debug)]
pub struct Game {
    pub board: Vec<Vec<Cell>>,
    pub player: Player,
    pub max_moves: u32,
    pub current_moves: u32,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, row) in self.board.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if i == self.player.y && j == self.player.x {
                    write!(f, "👨 ")?;
                } else {
                    match cell {
                        Cell::Wall    => write!(f, "🧱 ")?,
                        Cell::Empty   => write!(f, "🟩 ")?,
                        Cell::Food(_) => write!(f, "🍎 ")?,
                        Cell::Poison(_) => write!(f, "☠️  ")?,
                    }
                }
            }
            writeln!(f)?;
        }
        write!(f, "Forza: {}, Mosse: {}/{}", self.player.strength, self.current_moves, self.max_moves)
    }
}

impl Game {
    pub fn new(n: usize, m: usize, food_amount: u32, poison_amount: u32, initial_strength: u32, max_moves: u32) -> Game {
        let mut board = vec![vec![Cell::Empty; n]; n];
        for i in 0..n {
            board[0][i] = Cell::Wall;
            board[n - 1][i] = Cell::Wall;
            board[i][0] = Cell::Wall;
            board[i][n - 1] = Cell::Wall;
        }
        let mut rng = thread_rng();
        let mut positions: Vec<(usize, usize)> = (1..n - 1)
            .flat_map(|i| (1..n - 1).map(move |j| (i, j)))
            .collect();
        positions.shuffle(&mut rng);
        for i in 0..m {
            board[positions[i].0][positions[i].1] = Cell::Food(food_amount);
        }
        for i in m..2 * m {
            board[positions[i].0][positions[i].1] = Cell::Poison(poison_amount);
        }
        let player_pos = positions[2 * m];
        let player = Player::new(player_pos.1, player_pos.0, Direction::random(), initial_strength);
        Game { board, player, max_moves, current_moves: 0 }
    }

    /// Avanza di un passo. Restituisce `true` se il gioco continua, `false` se è finito.
    pub fn step(&mut self) -> bool {
        let mut rng = thread_rng();
        if !rng.gen_bool(0.5) {
            self.player.direction = Direction::random();
        }
        let (dx, dy) = match self.player.direction {
            Direction::Up    => (0i32, -1i32),
            Direction::Down  => (0,  1),
            Direction::Left  => (-1, 0),
            Direction::Right => (1,  0),
        };
        let new_x = self.player.x as i32 + dx;
        let new_y = self.player.y as i32 + dy;
        if new_x < 0 || new_x >= self.board.len() as i32
            || new_y < 0 || new_y >= self.board.len() as i32
        {
            self.player.direction = self.player.direction.opposite();
            return true;
        }
        let new_x = new_x as usize;
        let new_y = new_y as usize;
        match self.board[new_y][new_x] {
            Cell::Wall => {
                self.player.direction = self.player.direction.opposite();
            }
            Cell::Empty => {
                self.player.x = new_x;
                self.player.y = new_y;
            }
            Cell::Food(amount) => {
                self.player.strength += amount;
                self.board[new_y][new_x] = Cell::Empty;
                self.player.x = new_x;
                self.player.y = new_y;
            }
            Cell::Poison(amount) => {
                self.player.strength = self.player.strength.saturating_sub(amount);
                self.board[new_y][new_x] = Cell::Empty;
                self.player.x = new_x;
                self.player.y = new_y;
            }
        }
        self.current_moves += 1;
        self.player.strength > 0 && self.current_moves < self.max_moves
    }

    pub fn is_won(&self) -> bool {
        self.current_moves >= self.max_moves && self.player.strength > 0
    }

    pub fn is_lost(&self) -> bool {
        self.player.strength == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_creation() {
        let game = Game::new(10, 5, 10, 5, 100, 50);
        assert_eq!(game.board.len(), 10);
        assert_eq!(game.player.strength, 100);
        assert_eq!(game.max_moves, 50);
        assert_eq!(game.current_moves, 0);
    }

    #[test]
    fn test_game_boundaries() {
        let game = Game::new(5, 1, 10, 1, 100, 100);
        assert_eq!(game.board[0][0], Cell::Wall);
        assert_eq!(game.board[4][4], Cell::Wall);
        assert_eq!(game.board[0][4], Cell::Wall);
        assert_eq!(game.board[4][0], Cell::Wall);
    }

    #[test]
    fn test_game_win_condition() {
        let mut game = Game::new(5, 0, 0, 0, 100, 5);
        for _ in 0..10 {
            game.step();
        }
        assert!(game.is_won());
    }

    #[test]
    fn test_game_lose_condition() {
        let game = Game::new(5, 0, 0, 0, 1, 100);
        assert!(!game.is_lost());
        let mut zero_strength = game.clone();
        zero_strength.player.strength = 0;
        assert!(zero_strength.is_lost());
    }
}
