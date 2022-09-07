use crate::upgrades::Upgrade;
use core::cmp::Ordering;
use rug::{
    float::Round,
    ops::{MulAssignRound, MulFromRound},
    Float,
};
#[derive(Debug)]
struct F(f64);
impl MulFromRound<f64> for F {
    type Round = Round;
    type Ordering = Ordering;
    fn mul_from_round(&mut self, lhs: f64, round: Round) -> Ordering {
        let mut f = Float::with_val(53, lhs);
        let dir = f.mul_assign_round(self.0, round);
        self.0 = f.to_f64();
        dir
    }
}
pub enum BuyAmount {
    MAX,
    ONE,
    FIVE,
}
#[derive(Debug, Clone)]
pub struct PointGenerator {
    pub name: String,
    pub cost: Float,
    pub cost_multiplier: Float,
    pub amount: Float,
    pub points_generated: Float,
    pub owned_upgrades: Vec<Upgrade>,
}
impl PointGenerator {
    pub fn new(name: &str, cost: Float, cost_multiplier: f64, points_generated: Float) -> Self {
        PointGenerator {
            name: name.to_string(),
            cost,
            cost_multiplier: Float::with_val(53, cost_multiplier),
            points_generated,
            amount: Float::with_val(50, 0),
            owned_upgrades: Vec::new(),
        }
    }
    pub fn make_generators() -> Vec<PointGenerator> {
        let pgs = vec![
            PointGenerator::new(
                "Basic Generator+",
                Float::with_val(8, 1000),
                1.17,
                Float::with_val(10, 1),
            ),
            PointGenerator::new(
                "Basic Generator++",
                Float::with_val(10, 1e4),
                1.18,
                Float::with_val(10, 10),
            ),
            PointGenerator::new(
                "Basic Generator+++",
                Float::with_val(12, 1e5),
                1.19,
                Float::with_val(10, 100),
            ),
            PointGenerator::new(
                "Basic Generator#",
                Float::with_val(14, 1e6),
                1.20,
                Float::with_val(10, 1e3),
            ),
            PointGenerator::new(
                "Basic Generator#+",
                Float::with_val(16, 1e7),
                1.21,
                Float::with_val(50, 1e4),
            ),
            PointGenerator::new(
                "Basic Generator#++",
                Float::with_val(18, 1e8),
                1.22,
                Float::with_val(50, 1e5),
            ),
            PointGenerator::new(
                "Basic Generator#+++",
                Float::with_val(20, 1e9),
                1.23,
                Float::with_val(10, 1e6),
            ),
            PointGenerator::new(
                "Basic Generator##",
                Float::with_val(20, 1e10),
                1.24,
                Float::with_val(10, 1e7),
            ),
            PointGenerator::new(
                "Basic Generator##+",
                Float::with_val(20, 1e11),
                1.25,
                Float::with_val(10, 1e8),
            ),
            PointGenerator::new(
                "Basic Generator##++",
                Float::with_val(20, 1e12),
                1.26,
                Float::with_val(50, 1e9),
            ),
            PointGenerator::new(
                "Basic Generator##+++",
                Float::with_val(20, 1e13),
                1.27,
                Float::with_val(50, 1e10),
            ),
            PointGenerator::new(
                "Basic Generator###",
                Float::with_val(20, 1e13),
                1.27,
                Float::with_val(50, 1e11),
            ),
        ];
        pgs
    }
    pub fn increase_generator(generator: &mut PointGenerator) {
        generator.cost *= &generator.cost_multiplier;
        generator.amount += 1;
    }
}
