pub enum RetainedGuiEvent {
    Click(usize),
    Checkbox(usize, bool),
    IntegerChange(usize, i32),
}
