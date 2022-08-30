use std::io::{stdin, stdout, Write};

use anyhow::Result;
use dialoguer::MultiFuzzySelect;

fn main() -> Result<()> {
    let items = stdin().lines().collect::<Result<Vec<_>, _>>()?;

    let selected_idxs = MultiFuzzySelect::new()
        .report(false)
        .highlight_matches(true)
        .items(&items)
        .interact()?;

    let selected_items = items
        .iter()
        .enumerate()
        .filter(|(idx, _)| selected_idxs.contains(idx))
        .map(|(_, s)| format!("{s}\n"))
        .collect::<Vec<_>>();

    stdout().write_all(selected_items.join("").as_bytes())?;
    Ok(())
}
