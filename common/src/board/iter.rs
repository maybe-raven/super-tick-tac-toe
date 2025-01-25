use crate::{Board, BoardItem};

use super::BoardIndex;
use std::{iter::Enumerate, slice::Iter};

#[derive(Debug, Clone)]
pub struct BoardEnumerate<'a, T> {
    iter: Enumerate<Iter<'a, T>>,
}

impl<'a, T> From<&'a Board<T>> for BoardEnumerate<'a, T> {
    fn from(board: &'a Board<T>) -> Self {
        BoardEnumerate {
            iter: board.tiles.iter().enumerate(),
        }
    }
}

impl<'a, T> Iterator for BoardEnumerate<'a, T> {
    type Item = (BoardIndex, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let (index, item) = self.iter.next()?;
        let index = BoardIndex::try_from(index).expect(
            "Invariant: A board always contains exactly as many items as there are board indices.",
        );
        Some((index, item))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<T> ExactSizeIterator for BoardEnumerate<'_, T> {}

pub struct Unmarked<'a, T> {
    iter: BoardEnumerate<'a, T>,
}

impl<'a, T> From<&'a Board<T>> for Unmarked<'a, T> {
    fn from(board: &'a Board<T>) -> Self {
        Self { iter: board.into() }
    }
}

impl<'a, T> Iterator for Unmarked<'a, T>
where
    T: BoardItem,
{
    type Item = (BoardIndex, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.by_ref().find(|&item| item.1.is_markable())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.iter.size_hint().1)
    }
}
