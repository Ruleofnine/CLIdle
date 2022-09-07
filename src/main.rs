// use crate::{TICK_RATE,Error,Duration};
// use crate::crossterm::run;
// use crate::Player;
use std::{error::Error,time::Duration};
pub use clidle::{player::Player,TICK_RATE,crossterm::run,generators::PointGenerator,upgrades::Upgrade};
 fn main() -> Result<(),Box<dyn Error>> {
    print!("\x1B[2J\x1B[1;1H");
    let tick_rate = Duration::from_millis(TICK_RATE);
    let player = Player::new("Test");
    let generators = PointGenerator::make_generators();
    let mut upgrades =Upgrade::make_generator_upgrades(); 
    let upgrade_indexes=Upgrade::create_upgrade_indexes(&mut upgrades);
    run(tick_rate,true,player,generators,upgrades,upgrade_indexes)?;
    Ok(())

 }

