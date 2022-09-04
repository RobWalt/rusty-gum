use anyhow::{Error, Result};

#[derive(Debug)]
pub enum SelectMode {
    Single,
    Multi,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum MaybeUnbounded {
    Unbounded,
    Bounded(usize),
}

pub fn parse_usize(src: &str) -> Result<usize> {
    src.parse().map_err(Error::from)
}

pub fn parse_non_zero_usize(src: &str) -> Result<usize> {
    parse_usize(src).and_then(|val| {
        (val >= 1)
            .then_some(val)
            .ok_or_else(|| Error::msg("numeric value must be >= 1"))
    })
}

#[allow(dead_code)]
pub fn parse_optional_non_zero(src: &str) -> Result<MaybeUnbounded> {
    if src.to_lowercase().eq("none") {
        Ok(MaybeUnbounded::Unbounded)
    } else {
        parse_non_zero_usize(src).map(MaybeUnbounded::Bounded)
    }
}

pub fn parse_select_mode(src: &str) -> Result<SelectMode> {
    match src.to_lowercase() {
        s if s.eq("single") => Ok(SelectMode::Single),
        s if s.eq("multi") => Ok(SelectMode::Multi),
        _ => Err(Error::msg(
            "Only supported options are:\n\n\t- single\n\t-multi",
        )),
    }
}
