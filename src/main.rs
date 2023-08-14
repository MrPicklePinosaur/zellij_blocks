use std::{collections::HashMap, path::PathBuf};

use ansi_term::{Colour::Fixed, Style};
use zellij_tile::prelude::*;

#[derive(Default)]
struct State {
    tabs: Vec<TabInfo>,
    mode: InputMode,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self) {
        // attempt to read configuration from filesystem
        // NOTE, in future version, configurable plugins will be supported and we can get it
        // directly from the load arguments

        set_selectable(false);
        subscribe(&[
            EventType::ModeUpdate,
            EventType::TabUpdate,
            EventType::Timer,
        ]);
    }

    fn update(&mut self, event: Event) -> bool {
        let mut should_render = false;
        match event {
            Event::ModeUpdate(mode_info) => {
                self.mode = mode_info.mode;
                should_render = true;
            },
            Event::TabUpdate(tab_info) => {
                self.tabs = tab_info;
                should_render = true;
            },
            _ => (),
        };
        should_render
    }

    fn render(&mut self, rows: usize, cols: usize) {
        print!("BAR");
        print!(" ");
        print!("{:?}", self.mode);
        print!(" ");

        for tab in self.tabs.iter() {
            let tab_text = if tab.active {
                color_bold(GREEN, &tab.position.to_string())
            } else {
                tab.position.to_string()
            };
            print!("{tab_text}");
            print!(" ");
        }

        let datetime = chrono::offset::Local::now();
        print!(
            "{} {}",
            datetime.time().format("%H:%M").to_string(),
            datetime.date_naive().to_string()
        );
    }
}

pub const CYAN: u8 = 51;
pub const GRAY_LIGHT: u8 = 238;
pub const GRAY_DARK: u8 = 245;
pub const WHITE: u8 = 15;
pub const BLACK: u8 = 16;
pub const RED: u8 = 124;
pub const GREEN: u8 = 154;
pub const ORANGE: u8 = 166;

fn color_bold(color: u8, text: &str) -> String {
    format!("{}", Style::new().fg(Fixed(color)).bold().paint(text))
}
