use pride_cli::{apply_flag_color, Flag, StyleType};

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The flag to apply
    #[arg(short, long, default_value = "trans")]
    flag: String,

    /// The style to apply
    #[arg(short, long, default_value = "fg")]
    style: String,

    /// The grouping to apply
    #[arg(short, long)]
    grouping: usize,

    /// The text to apply the flag to
    text: String,
}

fn main() {
    let args = Args::parse();

    let flag = Flag::from(&args.flag);
    let style = StyleType::from(&args.style);

    println!(
        "{}",
        apply_flag_color(&args.text, flag, style, args.grouping)
    );
}
