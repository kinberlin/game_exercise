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
        assert_eq!(Cell::Wall, Cell::Wall);
        assert_eq!(Cell::Empty, Cell::Empty);
        assert_eq!(Cell::Food(10), Cell::Food(10));
        assert_ne!(Cell::Food(10), Cell::Food(20));
        assert_ne!(Cell::Wall, Cell::Empty);
    }

    #[test]
    fn test_cell_copy() {
        let cell1 = Cell::Food(5);
        let cell2 = cell1;
        assert_eq!(cell1, cell2);
    }
}
