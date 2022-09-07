use crate::{
    generators::{BuyAmount, PointGenerator},
    ui::StatefulList,
    upgrades::{Upgrade, UpgradeType},
    TPS,
};
use rug::{Assign, Float};

pub struct Player {
    pub name: String,
    pub points: Float,
    pub ppt: Float,
    pub pptmod: Float,
    pub ppc: Float,
    pub ppcmod: Float,
    pub pps: Float,
    pub prestige_points: Float,
    pub owned_pointgenerators: StatefulList<PointGenerator>,
    pub owned_upgrades: Vec<Upgrade>,
}

impl Player {
    pub fn new(new_name: &str) -> Player {
        Player {
            name: String::from(new_name),
            points: Float::with_val(50, 0),
            ppt: Float::with_val(50, 0),
            pptmod: Float::with_val(50, 1),
            ppc: Float::with_val(50, 1),
            ppcmod: Float::with_val(50, 1),
            pps: Float::with_val(50, 0),
            prestige_points: Float::new(10),
            owned_pointgenerators: StatefulList::with_items(vec![PointGenerator::new(
                "Basic Generator",
                Float::with_val(10, 100),
                1.17,
                Float::with_val(10, 0.25),
            )]),
            owned_upgrades: Vec::new(),
        }
    }
    pub fn calc_ppt(&mut self) {
        self.ppt = Float::new(10);
        for g in &self.owned_pointgenerators.items {
            let pta = &g.points_generated * &g.amount;
            self.ppt += pta;
        }
        self.ppt *= &self.pptmod
    }
    pub fn increase_points(&mut self) {
        self.points += &self.ppt
    }
    pub fn click_points(&mut self) {
        self.points += &self.ppc * &self.ppcmod
    }
    pub fn calc_pps(&mut self) {
        self.calc_ppt();
        let mut pps = Float::new(10);
        pps.assign(&self.ppt * TPS);
        self.pps = pps;
    }
    pub fn buy_generator_amount(
        &mut self,
        i: usize,
        buyamount: BuyAmount,
        all_generators: &mut Vec<PointGenerator>,
    ) {
        match buyamount {
            BuyAmount::MAX => {
                while self.points >= self.owned_pointgenerators.items[i].cost {
                    self.buy_generator(i);
                }
            }
            BuyAmount::ONE => {
                if self.points >= self.owned_pointgenerators.items[i].cost {
                    self.buy_generator(i);
                }
            }
            BuyAmount::FIVE => {
                for _ in 1..=5 {
                    if self.points >= self.owned_pointgenerators.items[i].cost {
                        self.buy_generator(i);
                    }
                }
            }
        };
        self.calc_pps();
        self.add_next_generator(all_generators);
    }
    pub fn buy_generator(&mut self, i: usize) {
        self.points -= &self.owned_pointgenerators.items[i].cost;
        PointGenerator::increase_generator(&mut self.owned_pointgenerators.items[i]);
    }
    pub fn add_next_generator(&mut self, generators: &mut Vec<PointGenerator>) {
        if self
            .owned_pointgenerators
            .items
            .iter()
            .rev()
            .take(1)
            .next()
            .unwrap()
            .amount
            > 0
        {
            if generators.get(0).is_some() {
                self.owned_pointgenerators.items.push(generators.remove(0));
            }
        }
    }
    pub fn buy_upgrade(
        &mut self,
        upgrades: &mut Vec<Vec<Upgrade>>,
        upgrade_indexes: &mut Vec<(usize, usize)>,
        selected_index: usize,
    ) -> bool {
        let upgrade_index = upgrade_indexes[selected_index];
        if self.points > upgrades[upgrade_index.0][upgrade_index.1].cost
            && self.owned_pointgenerators.items[0].amount > 0
        {
            let upgrade = upgrades[upgrade_index.0].remove(upgrade_index.1);
            self.points -= &upgrade.cost;
            if matches!(upgrade.upgradetype, UpgradeType::Generator) {
                self.owned_pointgenerators.items[upgrade_index.0].points_generated *=
                    &upgrade.number;
                self.owned_pointgenerators.items[upgrade_index.0]
                    .owned_upgrades
                    .push(upgrade);
                self.calc_pps();
            }
            true
        } else {
            false
        }
    }
}
