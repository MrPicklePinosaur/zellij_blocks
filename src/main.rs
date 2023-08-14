use std::{collections::HashMap, path::PathBuf};

use ansi_term::Colour::{Fixed, RGB};
use regex::Regex;
use zellij_tile::prelude::*;

#[derive(Default)]
struct State {
    tabs: Vec<TabInfo>,
    mode: InputMode,
    count: usize,
    session_name: Option<String>,
    palette: Palette,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self) {
        // attempt to read configuration from filesystem
        // NOTE, in future version, configurable plugins will be supported and we can get it
        // directly from the load arguments

        set_timeout(1.0);
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
                self.session_name = mode_info.session_name;
                self.palette = mode_info.style.colors;
                should_render = true;
            },
            Event::TabUpdate(tab_info) => {
                self.tabs = tab_info;
                should_render = true;
            },
            Event::Timer(_) => {
                // This is called at least once a second
                should_render = true;
                set_timeout(1.0);
                self.count += 1;
            },
            _ => (),
        };
        should_render
    }

    fn render(&mut self, rows: usize, cols: usize) {
        // TODO special formatting effects mess with padding

        let seperator = " ";
        let mut left_bar: Vec<String> = vec![];
        let mut center_bar: Vec<String> = vec![];
        let mut right_bar: Vec<String> = vec![];

        left_bar.push(" ".into());
        left_bar.push("Zellij".into());
        let mode_str = format!("{:?}", self.mode);
        left_bar.push(mode_style(&self.palette, &mode_str));

        let tab_seperator = " ";
        let tab_block = self
            .tabs
            .iter()
            .map(|tab| {
                let mut tab_text = format!("{}", tab.position);
                if tab.active {
                    tab_text = ansi_term::Style::new()
                        .fg(to_ansi(&self.palette.green))
                        .bold()
                        .paint(tab_text)
                        .to_string();
                };
                tab_text
            })
            .collect::<Vec<_>>()
            .join(tab_seperator);
        left_bar.push(tab_block);

        let datetime = chrono::offset::Local::now();
        let date_block = format!(
            "{} {}",
            datetime.time().format("%H:%M").to_string(),
            datetime.date_naive().to_string()
        );
        center_bar.push(date_block);

        // right_bar.push(self.count.to_string());

        if let Some(session_name) = &self.session_name {
            right_bar.push(format!("({})", session_name));
        }

        right_bar.push(" ".into());

        // Render Bar
        let left_bar_text = left_bar.join(seperator);
        let center_bar_text = center_bar.join(seperator);
        let right_bar_text = right_bar.join(seperator);

        let re = Regex::new(r#"\x1b\[[0-9;]*m"#).unwrap();
        let left_len = re.replace_all(&left_bar_text, "").chars().count();
        let center_len = re.replace_all(&center_bar_text, "").chars().count();
        let right_len = re.replace_all(&right_bar_text, "").chars().count();

        let left_padding = " ".repeat(cols / 2 - left_len - center_len / 2);
        let right_padding =
            " ".repeat(cols - left_len - left_padding.chars().count() - center_len - right_len);

        print!("{left_bar_text}{left_padding}{center_bar_text}{right_padding}{right_bar_text}")
    }
}

pub fn mode_style(palette: &Palette, mode: &str) -> String {
    let style = match mode {
        "Normal" => ansi_term::Style::new()
            .fg(to_ansi(&palette.black))
            .on(to_ansi(&palette.green))
            .bold(),
        _ => return mode.to_string(),
    };

    style.paint(mode).to_string()
}

pub fn div_up(a: usize, b: usize) -> usize {
    (0..a).step_by(b).size_hint().0
}

fn to_ansi(color: &PaletteColor) -> ansi_term::Color {
    match color {
        PaletteColor::Rgb((r, g, b)) => RGB(*r, *g, *b),
        PaletteColor::EightBit(bits) => Fixed(*bits),
    }
}

fn color_bold(color: u8, text: &str) -> String {
    format!(
        "{}",
        ansi_term::Style::new().fg(Fixed(color)).bold().paint(text)
    )
}
