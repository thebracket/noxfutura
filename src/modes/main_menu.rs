use crate::{GameMode, NoxMode, SharedResources};
use bengine::*;
use bengine::random::RandomNumberGenerator;

pub struct MainMenu {
    tagline: String,
}

impl MainMenu {
    pub fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let mut tagline = match rng.roll_dice(1, 8) {
            1 => "Histories",
            2 => "Chronicles",
            3 => "Sagas",
            4 => "Annals",
            5 => "Narratives",
            6 => "Recitals",
            7 => "Tales",
            8 => "Stories",
            _ => "",
        }
        .into();

        let first_noun = MainMenu::get_descriptive_noun(&mut rng);
        let mut second_noun = MainMenu::get_descriptive_noun(&mut rng);
        while first_noun == second_noun {
            second_noun = MainMenu::get_descriptive_noun(&mut rng);
        }

        tagline = format!("{} of {} and {}", tagline, first_noun, second_noun).to_string();

        Self { tagline }
    }

    const NOUNS: &'static [&'static str] = &[
        "Stupidity",
        "Idiocy",
        "Dullness",
        "Foolishness",
        "Futility",
        "Naievity",
        "Senselessness",
        "Shortsightedness",
        "Triviality",
        "Brainlessness",
        "Inanity",
        "Insensitivity",
        "Indiscretion",
        "Mindlessness",
        "Moronism",
        "Myopia",
        "Obtuseness",
        "Obliviousness",
        "Unthinkingness",
    ];

    const DEDICATION: &'static str =
        "To Kylah of the West and Jakie Monster -\nThe Bravest Little Warriors of Them All.";

    fn get_descriptive_noun(rng: &mut RandomNumberGenerator) -> String {
        rng.random_slice_entry(&MainMenu::NOUNS)
            .unwrap()
            .to_string()
    }
}

impl NoxMode for MainMenu {
    fn tick(&mut self, core: &mut Core, shared: &SharedResources) -> GameMode {
        use gui::*;

        let mut result = GameMode::MainMenu;
        shared.quad_render.render(shared.background_image, core);
        let ui = core.imgui;

        let size = get_window_size();
        let ver_string = ImString::new(format!("Nox Futura, {}", env!("CARGO_PKG_VERSION")));

        let copyright = gui::Window::new(&ver_string);
        copyright
            .position([10.0, size.height as f32 - 50.0], Condition::Always)
            .size([400.0, 400.0], Condition::FirstUseEver)
            .always_auto_resize(true)
            .collapsible(false)
            .build(ui, || {
                ui.text_colored(
                    [1.0, 1.0, 0.0, 1.0],
                    im_str!("(c) 2015-2020 Bracket Productions, All Rights Reserved."),
                )
            });

        let tagline_size = ui.calc_text_size(&ImString::new(&self.tagline), false, 500.0);
        let kylah_size = ui.calc_text_size(&ImString::new(MainMenu::DEDICATION), false, 500.0);
        let width = f32::max(tagline_size[0], kylah_size[0]);
        let hpos = (size.width as f32 / 2.0) - (width / 2.0);

        let mainmenu = gui::Window::new(im_str!("Main Menu"));
        mainmenu
            .position(
                [hpos, (size.height as f32 / 2.0) - 100.0],
                Condition::Always,
            )
            .always_auto_resize(true)
            .collapsible(false)
            .no_decoration()
            .build(ui, || {
                ui.text_colored([1.0, 1.0, 0.0, 1.0], &self.tagline);
                if ui.button(im_str!("New Game"), [100.0, 20.0]) {
                    result = GameMode::WorldGen1;
                }
                if std::path::Path::new("world.dat").exists() {
                    if ui.button(im_str!("Play/Continue Game"), [100.0, 20.0]) {
                        result = GameMode::PlayGame;
                    }
                }
                if ui.button(im_str!("Quit"), [100.0, 20.0]) {
                    result = GameMode::Quitting;
                }
                ui.text_colored([1.0, 0.0, 0.0, 1.0], &MainMenu::DEDICATION);
            });

        result
    }
}
