use std::collections::VecDeque;
use console_backend::Console;
use console_backend::{Color, colors};
use crate::theme::theme;

pub struct ScrollingMessageConsole {
    messages: VecDeque<(String, Color)>,
    console: Console,
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
    }

    pub fn render(&mut self, gl: &gl::Gl) {
        let clear: Color = Color::from_int(0, 0, 0, 0.0);
        self.console.clear();
        let mut current_height: i32 = (self.height - 1) as i32;
        for (message, color) in self.messages.iter() {
            let mut x = 0;
            for c in message.chars() {
                self.console.put_char(c, x, current_height as i32, *color, Some(*colors::CLEAR), 3);
                x += 1;
            }
            current_height -= 1;
        }
        self.console.render(&gl);
    }
}