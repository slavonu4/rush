use anyhow::Context;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref BENCODED_INT_REGEXP: Regex = Regex::new(r"i(?<value>\d+)e")
        .context("Unable to create bencoded integer pattern matcher")
        .unwrap();
    pub static ref BENCODED_STR_REGEXP: Regex = Regex::new(r"(?<len>\d+):(?<value>\S+)")
        .context("Unable to create bencoded integer pattern matcher")
        .unwrap();
}
