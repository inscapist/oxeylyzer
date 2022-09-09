#![feature(fs_try_exists)]
#![feature(exclusive_range_pattern)]
#![feature(let_chains)]

pub mod language_data;
pub mod load_text;
pub mod layout;
pub mod trigram_patterns;
pub mod utility;
pub mod weights;
pub mod analyze;
pub mod generate;
pub mod translation;
pub mod languages_cfg;
