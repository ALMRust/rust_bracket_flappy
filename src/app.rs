use bracket_lib::prelude::*;
use crate::types;

pub fn app() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(context, types::State::new())
}