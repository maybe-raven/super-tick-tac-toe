use super::BoardIndex;
use std::{iter::Enumerate, slice::Iter};

#[derive(Debug, Clone)]
pub(crate) struct BoardEnumerate<'a, T> {
    pub(super) iter: Enumerate<Iter<'a, T>>,
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
        (BoardIndex::N, Some(BoardIndex::N))
    }
}

impl<'a, T> ExactSizeIterator for BoardEnumerate<'a, T> {}
