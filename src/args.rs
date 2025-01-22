use clap::{Args, ValueEnum, Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct AutomataArgs {
    #[command(flatten)]
    pub window: WindowArgs,

    #[arg(
        short = 's',
        long = "cell-size",
        default_value_t = 5.0,
    )]
    pub cell_size: f32,

    #[arg(
        short = 'g',
        long = "gens-per-sec",
        default_value_t = 10.0,
    )]
    pub gens_per_sec: f32,

    #[command(subcommand)]
    pub cell: CellType,
}

#[derive(Args, Debug)]
#[group(
    required = true,
    multiple = true,
)]
pub struct WindowArgs {
    #[arg(
        short = 'f',
        long = "fullscreen",
        conflicts_with_all = ["width", "height"],
    )]
    pub fullscreen: bool,

    #[arg(
        short = 'x',
        long = "width",
        requires = "height",
    )]
    pub width: Option<f32>,

    #[arg(
        short = 'y',
        long = "height",
        requires = "width",
    )]
    pub height: Option<f32>,
}

#[derive(Subcommand, Debug)]
pub enum CellType {
    Life(PercentArg),
    Cyclic(CyclicArgs),
    Brain(PercentArg),
}

#[derive(Args, Debug)]
pub struct PercentArg {
    #[arg(
        short = 'p',
        long = "percentage",
        default_value_t = 50,
        value_parser = clap::value_parser!(u8).range(0..=100),
    )]
    pub percentage: u8,
}

#[derive(Args, Debug)]
pub struct CyclicArgs {
    #[arg(
        short = 't',
        long = "threshold",
        default_value_t = 1,
        value_parser = clap::value_parser!(u8).range(1..=8),
    )]
    pub threshold: u8,

    #[arg(
        value_enum,
        short = 'p',
        long = "palette",
        default_value_t = Palette::Grayscale,
    )]
    pub palette: Palette,
}

#[derive(Clone, ValueEnum, Debug)]
pub enum Palette {
    Rainbow,
    Grayscale,
}
