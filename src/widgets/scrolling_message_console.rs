use std::collections::VecDeque;
use console_backend::Console;
use console_backend::{Color, colors};
use crate::theme::theme;

pub struct ScrollingMessageConsole {
    messages: VecDeque<(String, Color)>,
    pub console: Console,
    height: u32,
}

impl ScrollingMessageConsole {
    pub fn new(console: Console, height: u32) -> Self {
        ScrollingMessageConsole {
            console,
            messages: VecDeque::new(),
            height,
        }
    }

    pub fn add_message(&mut self, message: &str) {
        self.add_colored_message(message, *theme::REGULAR_ALERT_TEXT);
    }

    pub fn add_colored_message(&mut self, message: &str, color: Color) {
        self.messages.push_back((message.to_string(), color));
        let extra = (self.messages.len() as i32) - (self.height as i32);

        if extra > 0 {
            for _ in 0..extra {
                self.messages.pop_front();
            }
        }
        self.refresh_buffer();
    }

    pub fn refresh_buffer(&mut self) {
        let clear: Color = Color::from_int(0, 0, 0, 0.0);
        self.console.clear();
        let x = "║═╔╗╚╝";
        let (width, height) = self.console.dimensions();
        let top = format!("╗{}╗", std::iter::repeat("═").take(width as usize - 2).collect::<String>());
        let bottom = format!("╚{}╝", std::iter::repeat("═").take(width as usize - 2).collect::<String>());
        self.console.put_text(&top, 0, self.height as i32 - 1, *colors::WHITE, Some(*colors::CLEAR), 3);
        let mut current_height: i32 = (self.height - 2) as i32;
        for (message, color) in self.messages.iter() {
            self.console.put_char('║', 0, current_height as i32, *colors::WHITE, Some(*colors::CLEAR), 3);
            self.console.put_text(message, 1 , current_height as i32, *color, Some(*colors::CLEAR), 3);
            self.console.put_char('║', width as i32 - 1, current_height as i32, *colors::WHITE, Some(*colors::CLEAR), 3);
            current_height -= 1;
        }
        for n in 1..=current_height {
            self.console.put_char('║', 0, n as i32, *colors::WHITE, Some(*colors::CLEAR), 3);
            self.console.put_char('║', width as i32 - 1, n as i32, *colors::WHITE, Some(*colors::CLEAR), 3);
        }
        self.console.put_text(&bottom, 0, 0, *colors::WHITE, Some(*colors::CLEAR), 3);
    }

    pub fn render(&mut self, gl: &gl::Gl) {
        self.console.render(&gl);
    }
}