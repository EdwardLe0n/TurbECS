use turbo::prelude::*;

use super::super::{component::Component, position::Position};

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct TextComponent {
    pub text : String,
    pub position : Position,
    color : u32,
    font : String
}

impl TextComponent {
    pub fn new(some_str : String) -> Component {

        let some_text = TextComponent {
            text: some_str, position : Position::new(), color : 0xffffffff, font: "medium".to_string()
        };

        return Component::Text(some_text);

    }
}

impl TextComponent {

    pub fn render(&self) {
        text!(
            &self.text,
            x = self.position.get_x(),
            y = self.position.get_y(),
            color = self.color,
            font = &self.font
        )
    }

}