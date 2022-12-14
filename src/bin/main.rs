use std::process::exit;

use anyhow::Result;
use rusty_gum::{exec_choose, exec_confirm, exec_filter, exec_input, RGApp, Subcommands};
use structopt::StructOpt;

fn main() -> Result<()> {
    let app_opts = RGApp::from_args();
    #[cfg(debug_assertions)]
    println!("{app_opts:?}");
    match app_opts.cmd {
        Subcommands::Choose(cmd_opts) => exec_choose(cmd_opts),
        Subcommands::Confirm(cmd_opts) => exec_confirm(cmd_opts),
        Subcommands::Filter(cmd_opts) => exec_filter(cmd_opts),
        Subcommands::Input(cmd_opts) => exec_input(cmd_opts),
        _ => todo!(),
    }?;
    exit(0)
}
