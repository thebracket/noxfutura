use bengine::gui::*;

pub fn begin_table(headings: &[&str], imgui: &Ui, table_id: &str, border: bool) {
    let id = format!("{}_h", table_id);
    imgui.columns(headings.len() as i32, &ImString::new(id), border);

    for h in headings {
        imgui.text_colored([0.1, 1.0, 1.0, 1.0], ImString::new(*h));
        imgui.next_column();
    }
    imgui.separator();
}

pub fn end_table(imgui: &Ui, id: &ImStr) {
    imgui.columns(1, id, false)
}
