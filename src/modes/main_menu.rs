use super::resources::SharedResources;
use imgui::*;
use bracket_random::prelude::*;

pub struct MainMenu {
    tagline : String
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
            _ => ""
        }.into();

        let first_noun = MainMenu::get_descriptive_noun(&mut rng);
        let mut second_noun = MainMenu::get_descriptive_noun(&mut rng);
        while first_noun == second_noun {
            second_noun = MainMenu::get_descriptive_noun(&mut rng);
        }

        tagline = format!("{} of {} and {}", tagline, first_noun, second_noun).to_string();

        Self {
            tagline
        }
    }

    const NOUNS : &'static[&'static str] = &["Stupidity", "Idiocy", "Dullness", "Foolishness", "Futility", "Naievity",
        "Senselessness", "Shortsightedness", "Triviality", "Brainlessness", "Inanity",
        "Insensitivity", "Indiscretion", "Mindlessness", "Moronism", "Myopia", "Obtuseness",
        "Obliviousness", "Unthinkingness"];

    const DEDICATION : &'static str = "To Kylah of the West and Jakie Monster -\nThe Bravest Little Warriors of Them All.";

    fn get_descriptive_noun(rng : &mut RandomNumberGenerator) -> String {
        rng.random_slice_entry(&MainMenu::NOUNS).unwrap().to_string()
    }

    pub fn tick(
        &mut self,
        resources: &SharedResources,
        frame: &wgpu::SwapChainOutput,
        context: &mut crate::engine::Context,
        ui: &imgui::Ui,
    ) -> super::ProgramMode {
        let mut result = super::ProgramMode::MainMenu;

        super::helpers::render_menu_background(context, frame, resources);

        let thanks = imgui::Window::new(im_str!("Thanks to our supporters"));
        thanks
            .position([context.size.width as f32 - 300.0, 125.0], Condition::Always)
            .size([400.0, 400.0], Condition::FirstUseEver)
            .always_auto_resize(true)
            .collapsible(false)
            .build(ui, || {
                ui.text(im_str!("Noah Bogart via Patreon"));
            });

        let copyright = imgui::Window::new(im_str!("Copyright"));
        copyright
            .position([10.0, context.size.height as f32 - 50.0], Condition::Always)
            .size([400.0, 400.0], Condition::FirstUseEver)
            .always_auto_resize(true)
            .collapsible(false)
            .build(ui, || {
                ui.text_colored([1.0, 1.0, 0.0, 1.0], im_str!("(c) 2015-2020 Bracket Productions, All Rights Reserved."))
            });

        let tagline_size = ui.calc_text_size(
            &ImString::new(&self.tagline),
            false, 
            500.0
        );
        let kylah_size = ui.calc_text_size(&ImString::new(MainMenu::DEDICATION), false, 500.0);
        let width = f32::max(tagline_size[0], kylah_size[0]);
        let hpos = (context.size.width as f32 / 2.0) - (width / 2.0);

        let mainmenu = imgui::Window::new(im_str!("Main Menu"));
        mainmenu
            .position([hpos, (context.size.height as f32 / 2.0) - 100.0], Condition::Always)
            .always_auto_resize(true)
            .collapsible(false)
            .no_decoration()
            .build(ui, || {
                ui.text_colored([1.0, 1.0, 0.0, 1.0], &self.tagline);
                ui.button(im_str!("New Game"), [100.0, 20.0]);
                if ui.button(im_str!("Quit"), [100.0, 20.0]) {
                    result = super::ProgramMode::Quit;
                }
                ui.text_colored([1.0, 0.0, 0.0, 1.0], &MainMenu::DEDICATION);
            });

        result
    }
}
