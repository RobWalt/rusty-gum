mod choose;
pub use choose::exec_choose;
use choose::ChooseArgs;

mod confirm;
pub use confirm::exec_confirm;
use confirm::ConfirmArgs;

mod filter;
pub use filter::exec_filter;
use filter::FilterArgs;

mod input;
pub use input::exec_input;
use input::InputArgs;

use anyhow::{Error, Result};
use std::io::Write;
use std::io::{stdin, stdout};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "RustyGum", about = "A tool for rusty shell scripts")]
pub struct RGApp {
    #[structopt(subcommand)]
    pub cmd: Subcommands,
}

#[derive(Debug, StructOpt)]
pub enum Subcommands {
    Choose(ChooseArgs),
    Confirm(ConfirmArgs),
    Filter(FilterArgs),
    Input(InputArgs),
    // TODO
    // Format(Format),
    // Join(Join),
    // Spin(Spin),
    // Style(Style),
    // Write(Write),
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Format some text for pretty output")]
pub struct Format {}

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
pub struct WriteArgs {}

fn get_stdin_items() -> Result<Vec<String>> {
    stdin()
        .lines()
        .map(|res| res.map_err(Error::from))
        .collect::<Result<Vec<_>>>()
}

fn pipe_to_stdout(selected_items: Vec<String>) -> Result<()> {
    selected_items
        .into_iter()
        .try_for_each(|item| writeln!(stdout(), "{item}"))
        .map_err(Error::from)
}

fn validate_items_non_empty(items: Vec<String>) -> Result<Vec<String>> {
    (!items.is_empty()).then_some(items).ok_or_else(|| {
        Error::msg("Input for `choose` was empty. Please provide at least 1 item via `stdin`!")
    })
}

fn select_index_items_from(items: Vec<String>) -> impl FnOnce(Vec<usize>) -> Vec<String> {
    move |indices| {
        items
            .into_iter()
            .enumerate()
            .filter(|(idx, _)| indices.contains(idx))
            .map(|(_, item)| item)
            .collect()
    }
}
