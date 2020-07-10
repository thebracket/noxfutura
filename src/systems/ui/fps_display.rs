use imgui::*;

pub fn fps_display(imgui: &Ui, frame_time: u128) {
    let title = format!(
        "Playing. Frame time: {} ms. FPS: {}. ### FPS",
        frame_time,
        imgui.io().framerate
    );
    let title_tmp = ImString::new(title);
    let window = imgui::Window::new(&title_tmp);
    window
        .collapsed(true, Condition::FirstUseEver)
        .size([300.0, 100.0], Condition::FirstUseEver)
        .movable(true)
        .position([0.0, 20.0], Condition::FirstUseEver)
        .build(imgui, || {});
}