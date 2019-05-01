use serde::Deserialize;

use crate::card::Card;
use crate::util::{uri::URI, UUID};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Component {
    Token,
    MeldPart,
    MeldResult,
    ComboPiece,
}

#[derive(Debug, Deserialize)]
pub struct RelatedCard {
    pub id: UUID,
    pub component: Component,
    pub name: String,
    pub type_line: String,
    pub uri: URI<Card>,
}
