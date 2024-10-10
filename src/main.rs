use pride::{apply_flag_color, Flag, StyleType};

use clap::Parser;
use clap_stdin::MaybeStdin;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The flag to apply
    #[arg(short, long, default_value = "trans", value_enum)]
    flag: Flag,

    /// The style to apply
    #[arg(short, long, default_value = "fg", value_enum)]
    style: StyleType,

    /// The grouping to apply
    #[arg(short, long, default_value = "0")]
    grouping: usize,

    /// The text to apply the flag to
    #[clap(default_value = "-")]
    text: MaybeStdin<String>,
}

fn main() {
    let args = Args::parse();

    println!(
        "{}",
        apply_flag_color(&args.text, args.flag, args.style, args.grouping)
    );
}
