use imgui::*;

pub fn lumberjack_display(imgui: &Ui) {
    let title = format!(
        "Lumberjack Mode. Click trees to designate for chopping. ### LumberJack",
    );
    let title_tmp = ImString::new(title);
    let window = imgui::Window::new(&title_tmp);
    window
        .collapsed(true, Condition::FirstUseEver)
        .no_inputs()
        .size([420.0, 100.0], Condition::FirstUseEver)
        .movable(false)
        .position([0.0, 20.0], Condition::FirstUseEver)
        .build(imgui, || {});
}
