use anyhow::{Error, Result};
use dialoguer::{Input, Password};
use structopt::StructOpt;

use super::pipe_to_stdout;

#[derive(Debug, StructOpt)]
#[structopt(about = "Prompt for single line input")]
pub struct InputArgs {
    #[structopt(
        long,
        default_value = "Input:",
        help = "Prompt to show in front of input"
    )]
    prompt: String,

    #[structopt(
        long,
        default_value = "Type something ...",
        help = "Hint to show in place of an empty input"
    )]
    hint_text: String,

    #[structopt(long, help = "Display chars in a hidden format, when used")]
    password: bool,
    // TODO: add this when it's supported by dialoguer
    //#[structopt(long, default_value = "", help = "Default value used as an input")]
    //default_input: String,

    //#[structopt(long, default_value = "400", help = "Limit the length of the input, none allowed for unbounded input", parse(try_from_str = parse_optional_non_zero))]
    //chars_limit: MaybeUnbounded,

    //#[structopt(long, default_value = "40", help = "Limit the amount of displayed chars of the input", parse(try_from_str = parse_non_zero_usize))]
    //display_width: usize,
}

pub fn exec_input(cmd_opts: InputArgs) -> Result<()> {
    match cmd_opts.password {
        true => Password::new()
            .with_prompt(cmd_opts.prompt)
            .report(false)
            .interact()
            .map_err(Error::from),
        false => Input::new()
            .with_prompt(cmd_opts.prompt)
            .default(cmd_opts.hint_text)
            .show_default(true)
            .allow_empty(true)
            .report(false)
            .interact()
            .map_err(Error::from),
    }
    .and_then(|input_text| pipe_to_stdout(vec![input_text]))
}
