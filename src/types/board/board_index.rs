#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BoardIndex {
    UpperLeft,
    UpperRight,
    LowerLeft,
    LowerRight,
    Up,
    Down,
    Left,
    Right,
    Center,
}

impl BoardIndex {
    pub(crate) const N: usize = 9;

    pub(crate) const ALL_LINES: [[BoardIndex; 3]; 8] = [
        // Rows
        [Self::UpperLeft, Self::Up, Self::UpperRight],
        [Self::Left, Self::Center, Self::Right],
        [Self::LowerLeft, Self::Down, Self::LowerRight],
        // Columns
        [Self::UpperLeft, Self::Left, Self::LowerLeft],
        [Self::Up, Self::Center, Self::Down],
        [Self::UpperRight, Self::Right, Self::LowerRight],
        // Diagonals
        [Self::UpperLeft, Self::Center, Self::LowerRight],
        [Self::LowerLeft, Self::Center, Self::UpperRight],
    ];
}

impl From<BoardIndex> for usize {
    fn from(value: BoardIndex) -> Self {
        match value {
            BoardIndex::UpperLeft => 0,
            BoardIndex::Up => 1,
            BoardIndex::UpperRight => 2,
            BoardIndex::Left => 3,
            BoardIndex::Center => 4,
            BoardIndex::Right => 5,
            BoardIndex::LowerLeft => 6,
            BoardIndex::Down => 7,
            BoardIndex::LowerRight => 8,
        }
    }
}

impl TryFrom<usize> for BoardIndex {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::UpperLeft),
            1 => Ok(BoardIndex::Up),
            2 => Ok(BoardIndex::UpperRight),
            3 => Ok(BoardIndex::Left),
            4 => Ok(BoardIndex::Center),
            5 => Ok(BoardIndex::Right),
            6 => Ok(BoardIndex::LowerLeft),
            7 => Ok(BoardIndex::Down),
            8 => Ok(BoardIndex::LowerRight),
            _ => Err(()),
        }
    }
}
