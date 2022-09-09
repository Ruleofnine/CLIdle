use crate::{
    generators::{BuyAmount, PointGenerator},
    ui::StatefulList,
    upgrades::{Upgrade, UpgradeType,MulitType},
    TPS,
};
use rug::Float;

pub struct Player {
    pub name: String,
    pub points: Float,
    pub ppt: Float,
    // pub ppc_cost_base: Float,
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
            points: Float::with_val(50, 1e100),
            ppt: Float::with_val(50, 0),
            // pptmod: Float::with_val(50, 1),
            ppc: Float::with_val(50, 1),
            // ppc_cost_base: Float::with_val(50,1),
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
            let points_to_add = Float::with_val(20, &g.base_points_generated);
            self.ppt += Float::with_val(20, &g.mod_points_generated * points_to_add) * &g.amount;
        }
    }
    pub fn increase_points(&mut self) {
        self.points += &self.ppt
    }
    pub fn click_points(&mut self) {
        for u in &self.owned_upgrades{
            self.ppcmod*=&u.number;
        }
        self.points += &self.ppc * &self.ppcmod
    }
    pub fn calc_pps(&mut self) {
        self.calc_ppt();
        self.pps = Float::with_val(50, &self.ppt * TPS);
    }
    pub fn calc_ppc(&mut self){
        self.ppc = Float::with_val(10,1);
        self.ppcmod = Float::with_val(10,1);
        for u in &self.owned_upgrades{
            match u.mulittype{
                MulitType::PointsBase =>{self.ppc+=&u.number},
                MulitType::PointsMulti=>{self.ppcmod*=&u.number},
                _=>{}
            }
            self.ppc = Float::with_val(20, &self.ppc*&self.ppcmod);
        }
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
        let mut upgrade_index = upgrade_indexes[selected_index];
        if self.points < upgrades[upgrade_index.0][upgrade_index.1].cost {
            return false;
        }
        let upgrade = upgrades[upgrade_index.0].remove(upgrade_index.1);
        match upgrade.upgradetype {
            UpgradeType::Click => {
                    self.points -= &upgrade.cost;
                    self.owned_upgrades.push(upgrade);
                    self.calc_ppc()
            }
            UpgradeType::Generator => {
                upgrade_index.0 -= 1;
                if self.owned_pointgenerators.items[upgrade_index.0].amount > 0 {
                    self.points -= &upgrade.cost;
                    self.owned_pointgenerators.items[upgrade_index.0].mod_points_generated *=
                        &upgrade.number;
                    self.owned_pointgenerators.items[upgrade_index.0]
                        .owned_upgrades
                        .push(upgrade);
                    self.calc_pps();
                }
            }
        }
        true
    }
}
