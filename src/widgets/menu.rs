use console_backend::{Console, ConsoleBuilder};
use console_backend::resources::Resources;
use std::path::Path;
use crate::theme::theme;
use glutin::{
    event::VirtualKeyCode,
};

pub struct Menu {
    pub console: Console,
    options: Vec<String>,
    header: String,
    height: u32,
}

impl Menu {
    pub fn new(gl: &gl::Gl,
               header: String,
               options: Vec<String>,
               width: u32,
               font_size: (f32, f32),
               relative_console: &Console) -> Self {
        let res = Resources::from_relative_exe_path(Path::new("assets")).unwrap();
        let height = options.len() as u32 + 2;
        let console = ConsoleBuilder::with_dimensions_and_font_size((width, height), font_size)
            .relative_to(relative_console)
            .font_from(relative_console)
            .centered(true)
            .layer(99)
            .build(&res, &gl)
            .unwrap();
        let mut menu = Menu {
            console,
            options,
            header,
            height,
        };
        menu.init_buffer();
        menu
    }

    pub fn init_buffer(&mut self) {
        self.console.clear();
        self.console.fill_background(*theme::BACKGROUND);
        self.console.put_text(&self.header, 0, (self.height - 1) as i32, *theme::REGULAR_ALERT_TEXT, None, 2);
        for (index, option) in self.options.iter().enumerate() {
            let menu_letter = (b'a' + index as u8) as char;
            let option = format!("({}) {}", menu_letter, option);
            let y = self.height as i32 - 2 - index as i32;
            self.console.put_text(&option, 0, y, *theme::REGULAR_ALERT_TEXT, None, 2);
        }
    }

    pub fn render(&mut self, gl: &gl::Gl) {
        self.console.render(&gl);
    }

    pub fn process_input(&mut self, key: VirtualKeyCode) -> Option<usize> {
        let string_key: String = format!("{:?}", key);
        if string_key.len() == 1 {
            let ch = string_key.chars().take(1).next().unwrap();
            let index = ch.to_ascii_lowercase() as usize - 'a' as usize;
            if index < self.options.len() {
                Some(index)
            } else {
                None
            }
        } else {
            None
        }
    }

}