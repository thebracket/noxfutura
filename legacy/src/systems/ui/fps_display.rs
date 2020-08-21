use imgui::*;

pub fn fps_display(imgui: &Ui, frame_time: u128) {
    let sz = crate::engine::DEVICE_CONTEXT.read().as_ref().unwrap().size;
    let title = format!(
        "Playing. Frame time: {:2} ms. FPS: {:.0}. ### FPS",
        frame_time,
        imgui.io().framerate
    );
    let title_tmp = ImString::new(title);
    let window = imgui::Window::new(&title_tmp);
    window
        .collapsed(true, Condition::FirstUseEver)
        .size([300.0, 100.0], Condition::FirstUseEver)
        .movable(true)
        .position([0.0, sz.height as f32 - 20.0], Condition::FirstUseEver)
        .build(imgui, || {});
}
