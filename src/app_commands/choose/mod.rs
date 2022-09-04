use std::io::{stdin, stdout, Write};

use crate::parsing::*;
use anyhow::{Error, Result};
use dialoguer::theme::ColorfulTheme;
use dialoguer::{MultiSelect, Select};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Choose from a list of items")]
pub struct Choose {
    #[structopt(long, default_value = "10", parse(try_from_str = parse_non_zero_usize), help = "Height of the list")]
    height: usize,

    #[structopt(
        long,
        default_value = "single",
        parse(try_from_str = parse_select_mode),
        help = "Use single/multi selection"
    )]
    mode: SelectMode,
    //
    // TODO: Add this when selection limit is available in dialoguer
    //#[structopt(
    //    long,
    //    default_value = "",
    //    help = "Additional indicator to highlight which element is hovered currently"
    //)]
    //cursor: String,

    //#[structopt(long, default_value = "[·]", help = "Prefix of currently hovered item")]
    //cursor_prefix: String,

    //#[structopt(long, default_value = "[x]", help = "Prefix of selected items")]
    //selected_prefix: String,

    //#[structopt(long, default_value = "[ ]", help = "Prefix of unselected items")]
    //unselected_prefix: String,

    //#[structopt(
    //    long,
    //    default_value = "1",
    //    parse(try_from_str = parse_optional_non_zero),
    //    help = "Maximum number of items to pick. Available options\n\t- None (unlimited)\n\t- [number]\n"
    //)]
    //limit: MaybeUnbounded,
}

pub fn exec_choose(choose_opts: Choose) -> Result<()> {
    get_stdin_items()
        .and_then(validate_items_non_empty)
        .and_then(choose_with_opts(choose_opts))
        .and_then(pipe_to_stdout)
}

fn get_stdin_items() -> Result<Vec<String>> {
    stdin()
        .lines()
        .map(|res| res.map_err(Error::from))
        .collect::<Result<Vec<_>>>()
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

fn choose_with_opts(choose_opts: Choose) -> impl FnOnce(Vec<String>) -> Result<Vec<String>> {
    move |items| {
        let theme = ColorfulTheme::default();

        match choose_opts.mode {
            SelectMode::Single => Select::with_theme(&theme)
                .items(&items)
                .max_length(choose_opts.height)
                .default(0)
                .interact()
                .map(|idx| vec![idx]),
            SelectMode::Multi => MultiSelect::with_theme(&theme)
                .items(&items)
                .max_length(choose_opts.height)
                .interact(),
        }
        .map_err(Error::from)
        .map(select_index_items_from(items))
    }
}

fn pipe_to_stdout(selected_items: Vec<String>) -> Result<()> {
    let mut stdout = stdout();
    selected_items
        .into_iter()
        .try_for_each(|item| writeln!(stdout, "{item}"))
        .map_err(Error::from)
}
