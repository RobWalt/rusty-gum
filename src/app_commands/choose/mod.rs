use crate::parsing::*;
use anyhow::{Error, Result};
use dialoguer::theme::ColorfulTheme;
use dialoguer::{MultiSelect, Select};
use structopt::StructOpt;

use super::{get_stdin_items, pipe_to_stdout, select_index_items_from, validate_items_non_empty};

#[derive(Debug, StructOpt)]
#[structopt(about = "Choose from a list of items")]
pub struct ChooseArgs {
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

pub fn exec_choose(choose_opts: ChooseArgs) -> Result<()> {
    get_stdin_items()
        .and_then(validate_items_non_empty)
        .and_then(choose_with_opts(choose_opts))
        .and_then(pipe_to_stdout)
}

fn choose_with_opts(choose_opts: ChooseArgs) -> impl FnOnce(Vec<String>) -> Result<Vec<String>> {
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
