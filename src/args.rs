use std::str::FromStr;
use clap::{Args, ValueEnum, Parser, Subcommand};
use crate::cell::life::LifeRule;

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
        short = 't',
        long = "threads",
        default_value_t = 4,
    )]
    pub threads: usize,

    #[arg(
        short = 'c',
        long = "chunks",
        default_value_t = 32,
    )]
    pub chunks: usize,

    #[arg(
        short = 'g',
        long = "gens-per-sec",
        default_value_t = 10,
    )]
    pub gens_per_sec: u32,

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
    Life(LifeArgs),
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
pub struct LifeArgs {
    #[arg(
        short = 'r',
        long = "rule",
        default_value = "B3S23",
        value_parser = parse_rule
    )]
    pub rule: LifeRule,

    #[clap(flatten)]
    pub percent_arg: PercentArg,
}

#[derive(Args, Debug)]
pub struct CyclicArgs {
    #[arg(
        short = 'n',
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

fn parse_rule(s: &str) -> Result<LifeRule, String> {
    LifeRule::from_str(s)
        .map_err(|rule_err| rule_err.message.to_owned())
}
