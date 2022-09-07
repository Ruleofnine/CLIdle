
use clidle::{
    generators::{BuyAmount, PointGenerator},
    utils::{check1k, format_number, IntIs},
    Player,
    upgrades::*,
    TICK_RATE
};
use rug::Float;
#[cfg(test)]
#[test]
fn player_starts_with_0_points() {
    let x = Player::new("test");
    println!("{}", x.points);
    assert!(x.points == 0)
}
#[test]
fn add_ppt() {
    let mut test = Player::new("test");
    test.ppt += Float::with_val(50, 100000);
    test.increase_points();
    let points = test.points.clone();
    assert!(points == test.points)
}
#[test]
fn add_ppc() {
    let mut test = Player::new("test");
    test.ppc += Float::with_val(50, 100000);
    test.click_points();
    let points = test.points.clone();
    assert!(points == test.points)
}
#[test]
fn create_building() {
    let mut pgs: Vec<PointGenerator> = Vec::new();
    let new = PointGenerator::new("1", Float::with_val(10, 1), 1.01, Float::with_val(10, 11));
    pgs.push(new);
    assert!(1 == pgs.len());
    assert!(1 == pgs[0].cost);
    assert!(11 == pgs[0].points_generated);
}
#[test]
fn create_building_vec() {
    let mut pgs: Vec<PointGenerator> = PointGenerator::make_generators();
    let new = PointGenerator::new(
        "test",
        Float::with_val(10, 999),
        1.01,
        Float::with_val(10, 11),
    );
    let key = pgs.len();
    pgs.insert(key, new);
    let test = pgs.iter().rev().take(1).next().unwrap();
    assert_eq!(test.cost, 999);
}
#[test]
fn test_1k() {
    let test = Float::with_val(10, 10);
    let bool_test = check1k(&test);
    assert!(matches!(bool_test, IntIs::Lesser));
    let test = Float::with_val(10, 1001);
    let bool_test = check1k(&test);
    assert!(matches!(bool_test, IntIs::Greater))
}
#[test]
fn test_format_number() {
    let test = Float::with_val(10, 10.8);
    let x = format_number(&test);
    assert_eq!(x, "11");
}
#[test]
fn buymax() {
    let mut player = Player::new("test");
    
    player.points += 10000;
    player.buy_generator_amount(0, BuyAmount::MAX,&mut PointGenerator::make_generators());
    assert_eq!(player.points, 657.5);
    assert_eq!(player.owned_pointgenerators.items[0].amount,18);
}
#[test]
fn buyone() {
    let mut player = Player::new("test");
    player.points += 100;
    player.buy_generator_amount(0, BuyAmount::ONE,&mut PointGenerator::make_generators());
    assert_eq!(player.points, 0);
    assert_eq!(player.owned_pointgenerators.items[0].amount, 1);
}
#[test]
fn buyfive() {
    let mut player = Player::new("test");
    player.points += 10000;
    player.buy_generator_amount(0, BuyAmount::FIVE,&mut PointGenerator::make_generators());
    assert_eq!(player.owned_pointgenerators.items[0].amount, 5);
}
#[test]
fn is_pps_correct(){
    let mut player = Player::new("test");
    player.points += 100000;
    let mut generators = PointGenerator::make_generators();
    player.buy_generator_amount(0, BuyAmount::ONE,&mut generators);
    let gen_pps = player.owned_pointgenerators.items[0].points_generated.clone()*1000/TICK_RATE*&player.owned_pointgenerators.items[0].amount;
    player.calc_pps();
    assert_eq!(player.pps,gen_pps);
    player.buy_generator_amount(0, BuyAmount::ONE,&mut generators);
    player.calc_pps();
    let gen_pps = player.owned_pointgenerators.items[0].points_generated.clone()*1000/TICK_RATE*&player.owned_pointgenerators.items[0].amount;
    assert_eq!(player.pps,gen_pps);
    player.buy_generator_amount(1, BuyAmount::ONE,&mut generators);
    player.calc_pps();
    let mut upgrades =Upgrade::make_generator_upgrades(); 
    let mut upgrade_indexes = Upgrade::create_upgrade_indexes(&mut upgrades);
    player.buy_upgrade(&mut upgrades,&mut upgrade_indexes,0);
    assert_eq!(player.owned_pointgenerators.items[0].owned_upgrades[0].name,"Gen 1, upgrade 1");
    // let gen_pps = player.owned_pointgenerators.items[0].points_generated.clone()*1000/TICK_RATE*&player.owned_pointgenerators.items[0].amount;
}
#[test]
fn buy_upgrade(){
    let mut player = Player::new("test");
    player.points += 1000;
    let mut generators = PointGenerator::make_generators();
    let mut upgrades =Upgrade::make_generator_upgrades(); 
    let mut upgrade_indexes = Upgrade::create_upgrade_indexes(&mut upgrades);
    player.buy_generator_amount(0, BuyAmount::ONE,&mut generators);
    player.buy_upgrade(&mut upgrades,&mut upgrade_indexes,0);
    assert_eq!(player.owned_pointgenerators.items[0].owned_upgrades[0].name,"Gen 1, upgrade 1")

}
#[test]
fn generator_as_long_as_upgrade_vec(){
    let upgrade_list =Upgrade::make_generator_upgrades(); 
    let generators = PointGenerator::make_generators();
    //Plus one becuase Player is given the first generator and is not in the starting vector
    assert_eq!(generators.len()+1,upgrade_list.len())
}
#[test]
fn upgrade_vec_vec_index(){
    let mut upgrades =Upgrade::make_generator_upgrades(); 
    let upgrade_indexs = Upgrade::create_upgrade_indexes(&mut upgrades);
    assert_eq!(upgrade_indexs[0],(0,0));
    assert_eq!(upgrade_indexs[1],(0,1));
    assert_eq!(upgrade_indexs[2],(1,0));
}
