use crate::Player;

pub trait BoardItem {
    fn is_markable(&self) -> bool;

    fn is_marked_by(&self, player: Player) -> bool;
}
