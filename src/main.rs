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
    let player = Player::new("Test");
    let generators = PointGenerator::make_generators();
    let upgrades = Upgrade::make_upgrades();
    // let upgrade_indexes = Upgrade::create_upgrade_indexes(&mut upgrades, &player);
    run(
        tick_rate,
        true,
        player,
        generators,
        upgrades,
        // upgrade_indexes,
    )?;
    Ok(())
}
