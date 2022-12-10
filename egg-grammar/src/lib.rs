use pest_derive::Parser;

pub const PEST_SOURCE: &str = include_str!("egg.pest");

#[derive(Debug, Parser)]
#[grammar = "egg.pest"]
pub struct EggAstParser;

pub use pest;
