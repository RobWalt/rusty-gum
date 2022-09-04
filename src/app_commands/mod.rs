mod choose;
pub use choose::exec_choose;
use choose::Choose;

mod confirm;
pub use confirm::exec_confirm;
use confirm::Confirm;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "RustyGum", about = "A tool for rusty shell scripts")]
pub struct RGApp {
    #[structopt(subcommand)]
    pub cmd: Subcommands,
}

#[derive(Debug, StructOpt)]
pub enum Subcommands {
    Choose(Choose),
    Confirm(Confirm),
    // TODO
    // Filter(Filter),
    // Format(Format),
    // Input(Input),
    // Join(Join),
    // Spin(Spin),
    // Style(Style),
    // Write(Write),
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Filter from a list of items")]
pub struct Filter {}

#[derive(Debug, StructOpt)]
#[structopt(about = "Format some text for pretty output")]
pub struct Format {}

#[derive(Debug, StructOpt)]
#[structopt(about = "Prompt for single line input")]
pub struct Input {}

#[derive(Debug, StructOpt)]
#[structopt(about = "Join text horizontally or vertically")]
pub struct Join {}

#[derive(Debug, StructOpt)]
#[structopt(about = "Show a spinner while processing a command")]
pub struct Spin {}

#[derive(Debug, StructOpt)]
#[structopt(about = "Apply colors, borders & spacing to text")]
pub struct Style {}

#[derive(Debug, StructOpt)]
#[structopt(about = "Prompt the user for multi-line input")]
pub struct Write {}
