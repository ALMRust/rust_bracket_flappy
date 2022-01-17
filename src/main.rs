#![warn(clippy::all, clippy::pedantic)]

mod types;
mod app;
mod consts;

use bracket_lib::prelude::*;
use crate::app::app;

fn main() -> BError {
    app()
}
