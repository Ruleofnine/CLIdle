// use crate::{TICK_RATE,Error,Duration};
// use crate::crossterm::run;
// use crate::Player;
pub use clidle::{
    crossterm::run, generators::PointGenerator, player::Player, upgrades::Upgrade, TICK_RATE,
};
use std::{error::Error, time::Duration};
fn main() -> Result<(), Box<dyn Error>> {
    print!("\x1B[2J\x1B[1;1H");
    let tick_rate = Duration::from_millis(TICK_RATE);
    run(
        tick_rate,
        true,
    )?;
    Ok(())
}
