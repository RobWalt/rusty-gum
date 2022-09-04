use crate::parsing::*;
use anyhow::{Error, Result};
use dialoguer::theme::ColorfulTheme;
use dialoguer::FuzzySelect;
use structopt::StructOpt;

use super::{get_stdin_items, pipe_to_stdout, select_index_items_from, validate_items_non_empty};

#[derive(Debug, StructOpt)]
#[structopt(about = "Filter from a list of items")]
pub struct FilterArgs {
    #[structopt(
        long,
        default_value = "single",
        parse(try_from_str = parse_select_mode),
        help = "Use single/multi selection"
    )]
    mode: SelectMode,

    #[structopt(
        long,
        default_value = "Searching for ... ",
        help = "Prompt to show in front of search term"
    )]
    prompt: String,
    // TODO: Add this when selection limit is available in dialoguer
    //#[structopt(long, default_value = "10", parse(try_from_str = parse_non_zero_usize), help = "Height of the list")]
    //height: usize,
    //
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

pub fn exec_filter(filter_opts: FilterArgs) -> Result<()> {
    get_stdin_items()
        .and_then(validate_items_non_empty)
        .and_then(filter_with_opts(filter_opts))
        .and_then(pipe_to_stdout)
}

fn filter_with_opts(filter_opts: FilterArgs) -> impl FnOnce(Vec<String>) -> Result<Vec<String>> {
    move |items| {
        let theme = ColorfulTheme::default();

        match filter_opts.mode {
            SelectMode::Single => FuzzySelect::with_theme(&theme)
                .items(&items)
                .report(false)
                .with_prompt(filter_opts.prompt)
                .default(0)
                .interact()
                .map(|idx| vec![idx]),
            // TODO add this when PR is merged
            SelectMode::Multi => unimplemented!("Not implemented yet"),
        }
        .map_err(Error::from)
        .map(select_index_items_from(items))
    }
}
