#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cell {
    Wall,
    Empty,
    Food(u32),
    Poison(u32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_equality() {
        assert_eq!(Cell::Food(10), Cell::Food(10));
        assert_ne!(Cell::Food(10), Cell::Food(5));
        assert_ne!(Cell::Food(10), Cell::Poison(10));
    }

    #[test]
    fn test_cell_copy() {
        let c = Cell::Food(5);
        let d = c;
        assert_eq!(c, d);
    }
}
