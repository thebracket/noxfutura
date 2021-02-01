use bracket_terminal::prelude::*;

pub struct ColorMenuItem {
    pub order: u32,
    pub label: String,
}

pub struct ColorMenu {
    menu_items: Vec<ColorMenuItem>,
    selected: ColorPair,
    unselected: ColorPair,
    selection: usize,
    chosen: bool,
}

impl ColorMenu {
    pub fn new(menu_items: Vec<ColorMenuItem>) -> Self {
        let mut new_menu = Self {
            menu_items,
            selected: ColorPair::new(WHITE, BLUE),
            unselected: ColorPair::new(WHITE, BLACK),
            selection: 0,
            chosen: false,
        };
        new_menu.menu_items.sort_by(|a, b| a.order.cmp(&b.order));
        new_menu
    }

    pub fn render(&self, ctx: &mut BTerm, left: u32, right: u32, top: u32) {
        let mut y = top;
        let center = left + ((right - left) / 2);
        let width = (right - left) as usize;

        let tmp = format!("{:width$}", " ", width = width);
        self.menu_items.iter().enumerate().for_each(|(i, mi)| {
            if i == self.selection {
                ctx.print_color_centered_at(center, y, self.selected.fg, self.selected.bg, &tmp);
                ctx.print_color_centered_at(
                    center,
                    y,
                    self.selected.fg,
                    self.selected.bg,
                    &mi.label,
                );
            } else {
                ctx.print_color_centered_at(
                    center,
                    y,
                    self.unselected.fg,
                    self.unselected.bg,
                    &tmp,
                );
                ctx.print_color_centered_at(
                    center,
                    y,
                    self.unselected.fg,
                    self.unselected.bg,
                    &mi.label,
                );
            }
            y += 1;
        });
    }

    pub fn handle_key(&mut self, key: &Option<VirtualKeyCode>) {
        match key {
            Some(VirtualKeyCode::Up) => {
                if self.selection == 0 {
                    self.selection = self.menu_items.len() - 1;
                } else {
                    self.selection -= 1;
                }
            }
            Some(VirtualKeyCode::Down) => {
                self.selection += 1;
                if self.selection == self.menu_items.len() {
                    self.selection = 0;
                }
            }
            Some(VirtualKeyCode::Space) => self.chosen = true,
            _ => {}
        }
    }

    pub fn handle_mouse(&mut self, ctx: &BTerm, left: u32, right: u32, top: u32) {
        let mouse = ctx.mouse_pos();
        if mouse.0 >= left as i32
            && mouse.0 <= right as i32
            && mouse.1 >= top as i32
            && mouse.1 < top as i32 + self.menu_items.len() as i32
        {
            self.selection = (mouse.1 - top as i32) as usize;
            if ctx.left_click {
                self.chosen = true;
            }
        }
    }

    pub fn get_selection(&mut self) -> Option<usize> {
        let chosen = self.chosen;
        self.chosen = false;
        if !chosen {
            None
        } else {
            Some(self.selection)
        }
    }
}
