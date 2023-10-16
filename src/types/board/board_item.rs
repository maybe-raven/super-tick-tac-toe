use crate::types::Player;

pub(crate) trait BoardItem {
    fn is_markable(&self) -> bool;

    fn is_marked_by(&self, player: Player) -> bool;
}
