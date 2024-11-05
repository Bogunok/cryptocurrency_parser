use pest::Parser;
use pest_derive::Parser;
use anyhow::anyhow;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct SolanaParser;
