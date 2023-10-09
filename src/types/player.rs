#[allow(dead_code)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Player {
    #[default]
    Circle, // Always goes first.
    Cross,
}
