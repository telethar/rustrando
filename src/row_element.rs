use crate::map_element;
use map_element::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RowElementState {
    NONE,
    CROSSED,
    CIRCLED
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowElement {
    pub kind: MapElementKind,
    pub state: RowElementState,
    pub min_count: Option<i32>,
    pub max_count: Option<i32>,
    pub notes: String
}

pub fn map_element_kind_to_row_element(kind: MapElementKind) -> RowElement {
    return RowElement {
       kind: kind,
       state: RowElementState::NONE,
       min_count: None,
       max_count: None,
       notes: String::new()
    };
}