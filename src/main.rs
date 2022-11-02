mod akpull;
use clap::{ValueEnum, Parser};
use crate::akpull::{AkPullArgs, AccelType, dp_noaccel};

#[derive(Clone, Debug, ValueEnum)]
enum BannerType {
  Standard,
  Limited,
  Event
}

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
  // Simple options

  #[clap(short, long, action)]
  verbose: bool,

  #[clap(short, long, value_parser, default_value = "standard")]
  banner: BannerType,

  #[clap(short, long, value_parser, default_value = "100")]
  pulls: u64,

  #[clap(short, long, value_parser, default_value = "10")]
  max_count: u32,

  // Advanced options

  #[clap(long, value_parser, default_value = "none")]
  accel: AccelType,

  #[clap(long, value_parser, default_value = "0.02")]
  base_rate: f64,

  #[clap(long, value_parser)]
  banner_rate: Option<f64>,

  #[clap(long, value_parser)]
  n_banner: Option<u32>
}

fn main() {
  let args = Args::parse();
  let pull_args = AkPullArgs {
    pulls: args.pulls,
    max_count: args.max_count,
    base_rate: args.base_rate
  };
  let res = dp_noaccel(pull_args);
  println!("WIP output:");
  for i in 0..args.max_count as usize {
    println!("{}x: {}", i, 100.0 * res.n6[i] as f64 / (u32::MAX as f64));
  }
  println!(">{}x: {}", args.max_count - 1, 100.0 * res.n6[args.max_count as usize] as f64 / (u32::MAX as f64));
}
