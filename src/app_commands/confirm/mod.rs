use std::process::exit;

use anyhow::{Error, Result};
use dialoguer::Confirm;
use structopt::StructOpt;

#[derive(Debug)]
pub enum ConfirmDefault {
    Confirm,
    Decline,
}

fn confirm_default_from_str(src: &str) -> Result<ConfirmDefault> {
    match src.to_lowercase() {
        s if s.eq("confirm") => Ok(ConfirmDefault::Confirm),
        s if s.eq("decline") => Ok(ConfirmDefault::Decline),
        _ => Err(Error::msg(
            "Invalid option, use either `confirm` or `decline`",
        )),
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Ask a user to confirm an action")]
pub struct ConfirmArgs {
    #[structopt(
        default_value = "Are you sure?",
        help = "Prompt question that needs to be confirmed"
    )]
    prompt: String,

    #[structopt(long, default_value = "confirm", parse(try_from_str = confirm_default_from_str), help = "The default confirmation action selected")]
    default_hover: ConfirmDefault,
    //
    // TODO add this when supported by dialoguer
    //#[structopt(
    //    long,
    //    default_value = "Yes",
    //    help = "The title of the affirmative button"
    //)]
    //affirmative: String,

    //#[structopt(long, default_value = "No", help = "The title of the negative button")]
    //negative: String,
}

pub fn exec_confirm(confirm_opts: ConfirmArgs) -> Result<()> {
    Confirm::new()
        .wait_for_newline(true)
        .report(false)
        .with_prompt(&confirm_opts.prompt)
        .default(match confirm_opts.default_hover {
            ConfirmDefault::Confirm => true,
            ConfirmDefault::Decline => false,
        })
        .interact()
        .map_err(Error::from)
        .map(continue_with_confirm_action)
}

// this function is technically `-> !` but it doesn't go well with the rest
fn continue_with_confirm_action(confirm_action: bool) {
    if confirm_action {
        exit(0)
    } else {
        exit(1)
    }
}
