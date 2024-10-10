use clap::ValueEnum;
use nu_ansi_term::{Color::Rgb, Style};

#[derive(Debug, Default, Clone, Copy, ValueEnum)]
pub enum Flag {
    Pride,
    Gay,
    Bisexual,
    Lesbian,
    Enby,
    #[default]
    Trans,
}

#[derive(Debug, Default, Clone, Copy, ValueEnum)]
pub enum StyleType {
    Bg,
    #[default]
    Fg,
}

pub struct Color(u8, u8, u8);

impl Color {
    pub fn to_style(&self) -> Style {
        Rgb(self.0, self.1, self.2)
    }
    pub fn is_dark(&self) -> bool {
        let luminance = 0.2126 * f64::from(color.0)
            + 0.7152 * f64::from(color.1)
            + 0.0722 * f64::from(color.2);
        luminance < 128.0
    }
}

type Colors = Vec<(u8, u8, u8)>;

pub fn get_flag_color(flag: Flag) -> Colors {
    match flag {
        Flag::Pride => vec![
            (255, 0, 0),
            (255, 165, 0),
            (255, 255, 0),
            (0, 128, 0),
            (0, 0, 255),
            (75, 0, 130),
            (238, 130, 238),
        ],
        Flag::Gay => vec![
            (32, 142, 113),
            (62, 207, 171),
            (157, 233, 195),
            (255, 255, 255),
            (129, 172, 225),
            (86, 69, 201),
            (62, 21, 118),
        ],
        Flag::Bisexual => vec![(214, 2, 112), (155, 79, 150), (0, 56, 168)],
        Flag::Lesbian => vec![
            (212, 44, 0),
            (253, 152, 85),
            (255, 255, 255),
            (209, 97, 162),
            (162, 1, 97),
        ],
        Flag::Enby => vec![
            (252, 244, 52),
            (255, 255, 255),
            (156, 89, 209),
            (44, 44, 44),
        ],
        Flag::Trans => vec![
            (91, 206, 250),
            (245, 169, 184),
            (255, 255, 255),
            (245, 169, 184),
            (91, 206, 250),
        ],
    }
}

fn style_line(text: &str, flag: Flag, style: StyleType, grouping: usize) -> String {
    let colors: Colors = get_flag_color(flag);
    let mut styled_text = String::new();

    for (i, c) in text.chars().enumerate() {
        let color: Color = &colors[(i / fin_grouping) % colors.len()];
        let styled_char = match style {
            StyleType::Bg => {
                // Choose black or white for fg based on luminance
                let fg_color = if color.is_dark() {
                    Rgb(255, 255, 255) // Light text for dark background
                } else {
                    Rgb(0, 0, 0) // Dark text for light background
                };

                &Style::new()
                    .on(color.to_style())
                    .fg(fg_color)
                    .paint(c.to_string())
                    .to_string()
            }
            StyleType::Fg => &Style::new()
                .fg(color.to_style())
                .paint(c.to_string())
                .to_string(),
        };
        styled_text.push_str(styled_char);
    }
    styled_text
}

pub fn apply_flag_color(text: &str, flag: Flag, style: StyleType, grouping: usize) -> String {
    let lines = text.split('\n');
    let mut styled_text = String::new();

    let fin_grouping = if grouping == 0 {
        let max_len = 0;
        for line in lines {
            if line.len() > max_len {
                max_len = line.len();
            }
        }

        if max_len >= colors.len() {
            max_len / colors.len()
        } else {
            1
        }
    } else {
        grouping
    };

    for line in lines {
        let styled_line = style_line(line, flag, style, fin_grouping);
        styled_text.push_str(&styled_line);
        styled_text.push('\n');
    }
    styled_text
}
