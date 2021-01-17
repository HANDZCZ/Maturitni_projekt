use fancy_regex::Regex;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref GAME_NAME_REGEX: Regex =
        Regex::new(r#"^(?!.*([ &_\-',\.])\1)[a-zA-Z0-9 &_\-',\.]{5,15}$"#).unwrap();
}

#[derive(Serialize, Deserialize, Default)]
pub struct GameData {
    pub field: Vec<Croft>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct Croft {
    pub x: u8,
    pub y: u8,
}
