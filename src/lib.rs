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

pub fn get_flag_color(flag: Flag) -> Vec<(u8, u8, u8)> {
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

pub fn apply_flag_color(text: &str, flag: Flag, style: StyleType, grouping: usize) -> String {
    let colors = get_flag_color(flag);
    let mut styled_text = String::new();

    for (i, c) in text.chars().enumerate() {
        let color = &colors[(i / grouping) % colors.len()];
        let styled_char = match style {
            StyleType::Bg => {
                let luminance = 0.2126 * (color.0 as f64)
                    + 0.7152 * (color.1 as f64)
                    + 0.0722 * (color.2 as f64);

                // Choose black or white for fg based on luminance
                let fg_color = if luminance > 128.0 {
                    Rgb(0, 0, 0) // Dark text for light background
                } else {
                    Rgb(255, 255, 255) // Light text for dark background
                };

                &Style::new()
                    .on(Rgb(color.0, color.1, color.2))
                    .fg(fg_color)
                    .paint(c.to_string())
                    .to_string()
            }
            StyleType::Fg => &Style::new()
                .fg(Rgb(color.0, color.1, color.2))
                .paint(c.to_string())
                .to_string(),
        };
        styled_text.push_str(styled_char);
    }
    styled_text
}
