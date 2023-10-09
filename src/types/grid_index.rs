use std::ops::{Index, IndexMut};

pub(crate) type GridArray<T> = [T; GridIndex::N];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GridIndex {
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

impl GridIndex {
    pub(crate) const N: usize = 9;
}

impl From<GridIndex> for usize {
    fn from(value: GridIndex) -> Self {
        match value {
            GridIndex::UpperLeft => 0,
            GridIndex::Up => 1,
            GridIndex::UpperRight => 2,
            GridIndex::Left => 3,
            GridIndex::Center => 4,
            GridIndex::Right => 5,
            GridIndex::LowerLeft => 6,
            GridIndex::Down => 7,
            GridIndex::LowerRight => 8,
        }
    }
}

impl TryFrom<usize> for GridIndex {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::UpperLeft),
            1 => Ok(GridIndex::Up),
            2 => Ok(GridIndex::UpperRight),
            3 => Ok(GridIndex::Left),
            4 => Ok(GridIndex::Center),
            5 => Ok(GridIndex::Right),
            6 => Ok(GridIndex::LowerLeft),
            7 => Ok(GridIndex::Down),
            8 => Ok(GridIndex::LowerRight),
            _ => Err(()),
        }
    }
}

impl<T> Index<GridIndex> for GridArray<T> {
    type Output = T;

    fn index(&self, index: GridIndex) -> &Self::Output {
        self.index(usize::from(index))
    }
}

impl<T> IndexMut<GridIndex> for GridArray<T> {
    fn index_mut(&mut self, index: GridIndex) -> &mut Self::Output {
        self.index_mut(usize::from(index))
    }
}
