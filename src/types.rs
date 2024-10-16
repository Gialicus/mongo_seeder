use clap::Parser;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub db: String,
    pub collections: Vec<CollectionConfig>,
}

fn default_number_of_items() -> usize {
    10
}

fn default_number_of_children() -> usize {
    3
}

#[derive(Debug, Deserialize)]
pub struct CollectionConfig {
    pub name: String,
    #[serde(default = "default_number_of_items")]
    pub number_of_items: usize,
    #[serde(default = "default_number_of_children")]
    pub number_of_children: usize,
    pub schema: HashMap<String, serde_json::Value>,
}

#[derive(Parser)]
#[command(name = "DB Populator")]
#[command(about = "Popola MongoDB con dati generati secondo uno schema JSON", long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub config: String,
    #[arg(short, long)]
    pub url: String,
}
