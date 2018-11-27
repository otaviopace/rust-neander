use ui::list_action::ListActions;

pub enum BlockLists {
    Operators,
    Variables,
}

pub struct ListState {
    pub current_line: usize,
    pub first_line: usize,
    pub last_line: usize,
    pub handle_action: Box<ListActions>,
}

pub struct UIState {
    pub block_selected: BlockLists,
    pub memory_list_operators: ListState,
    pub memory_list_variables: ListState,
    pub is_typing: bool,
    pub typing_char: Option<char>,
    pub quit: bool,
}

impl UIState {
    pub fn current_list(&self) -> &ListState {
        match self.block_selected {
            BlockLists::Operators => &self.memory_list_operators,
            BlockLists::Variables => &self.memory_list_variables,
        }
    }

    pub fn mutable_current_list(&mut self) -> &mut ListState {
        match self.block_selected {
            BlockLists::Operators => &mut self.memory_list_operators,
            BlockLists::Variables => &mut self.memory_list_variables,
        }
    }
}
