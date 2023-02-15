pub enum UserInput {
    Quit,
    MenuSelection(i8),
}

impl UserInput {
    pub fn is_continue(&self) -> bool {
        match self {
            UserInput::Quit => false,
            _ => true
        }
    }

    pub fn get_selection(&self) -> i8 {
        match self {
            UserInput::MenuSelection(selection) => *selection,
            _ => 0
        }
    }
}