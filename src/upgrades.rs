use crate::player::Player;
use rug::Float;
use tui::widgets::{ListState,ListItem};

#[derive(Clone, Debug)]
pub enum MulitType {
    PointsMulti,
    PointsBase,
    CostDiv,
}
#[derive(Clone, Debug)]
pub enum UpgradeType {
    Click,
    Generator,
}

pub struct UpgradeState {
    pub state: ListState,
    pub max_index: usize,
    pub upgrade_indexes: Vec<(usize, usize)>,
}

impl UpgradeState {
    pub fn new() -> UpgradeState {
        UpgradeState {
            state: ListState::default(),
            max_index: 0,
            upgrade_indexes: Vec::new(),
        }
    }
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if self.max_index == 0 {
                    0
                } else if i >= self.max_index - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if self.max_index == 0 {
                    0
                } else if i == 0 {
                    self.max_index - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    pub fn bought(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if self.max_index == 0 || i == 0 {
                    0
                } else if i == self.max_index - 1 {
                    i - 1
                } else {
                    i
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}
#[derive(Clone, Debug)]
pub struct Upgrade {
    pub name: String,
    pub cost: Float,
    pub mulittype: MulitType,
    pub upgradetype: UpgradeType,
    pub number: Float,
}
impl Upgrade {
    pub fn new(
        name: &str,
        cost: Float,
        mulittype: MulitType,
        upgradetype: UpgradeType,
        number: Float,
    ) -> Upgrade {
        Upgrade {
            name: name.to_string(),
            cost,
            mulittype,
            upgradetype,
            number,
        }
    }
    pub fn create_upgrade_indexes(
        vec_vec_upgrades: &mut Vec<Vec<Upgrade>>,
        player: &Player,
    ) -> Vec<(usize, usize)> {
        let mut upgrade_indexes: Vec<(usize, usize)> = Vec::new();
        for (vector_index, vector) in vec_vec_upgrades.iter().enumerate() {
            for (upgrade_index, upgrade) in vector.iter().enumerate() {
                if Upgrade::near_cost(&player.points, &upgrade.cost) {
                    upgrade_indexes.push((vector_index, upgrade_index))
                }
            }
        }
        upgrade_indexes
    }
    pub fn near_cost(points: &Float, cost: &Float) -> bool {
        if &Float::with_val(30, points * 3000) > cost {
            true
        } else {
            false
        }
    }
    pub fn make_upgrades() -> Vec<Vec<Upgrade>> {
        let keypress_upgrades = Upgrade::make_keypress_upgrades();
        let mut generator_upgrades = Upgrade::make_generator_upgrades();
        generator_upgrades[0] = keypress_upgrades;
        generator_upgrades
    }
    fn make_keypress_upgrades() -> Vec<Upgrade> {
        vec![
            Upgrade::new(
                "Key Upgrade 1",
                Float::with_val(10, 10),
                MulitType::PointsMulti,
                UpgradeType::Click,
                Float::with_val(10, 2),
            ),
            Upgrade::new(
                "Key Upgrade 2",
                Float::with_val(10, 100),
                MulitType::PointsMulti,
                UpgradeType::Click,
                Float::with_val(10, 2),
            ),
            Upgrade::new(
                "Key Upgrade 3",
                Float::with_val(10, 5000),
                MulitType::PointsMulti,
                UpgradeType::Click,
                Float::with_val(10, 2),
            ),
            Upgrade::new(
                "Key Upgrade 4",
                Float::with_val(10, 1e102),
                MulitType::PointsMulti,
                UpgradeType::Click,
                Float::with_val(10, 2),
            ),
        ]
    }
    fn make_generator_upgrades() -> Vec<Vec<Upgrade>> {
        let generator_vec_of_vecs = vec![
            Vec::new(),
            vec![
                Upgrade::new(
                    "Gen 1, upgrade 1",
                    Float::with_val(20, 10),
                    MulitType::PointsMulti,
                    UpgradeType::Generator,
                    Float::with_val(20, 2),
                ),
                Upgrade::new(
                    "Gen 1, upgrade 2",
                    Float::with_val(20, 10),
                    MulitType::PointsMulti,
                    UpgradeType::Generator,
                    Float::with_val(20, 2),
                ),
            ],
            vec![
                Upgrade::new(
                    "Gen 2, upgrade 1",
                    Float::with_val(20, 1e101),
                    MulitType::PointsMulti,
                    UpgradeType::Generator,
                    Float::with_val(20, 2),
                ),
                Upgrade::new(
                    "Gen 2, upgrade 2",
                    Float::with_val(20, 10),
                    MulitType::PointsMulti,
                    UpgradeType::Generator,
                    Float::with_val(20, 2),
                ),
            ],
            vec![
                Upgrade::new(
                    "Gen 3, upgrade 1",
                    Float::with_val(20, 10),
                    MulitType::PointsMulti,
                    UpgradeType::Generator,
                    Float::with_val(20, 2),
                ),
                Upgrade::new(
                    "Gen 3, upgrade 2",
                    Float::with_val(20, 10),
                    MulitType::PointsMulti,
                    UpgradeType::Generator,
                    Float::with_val(20, 2),
                ),
            ],
            vec![
                Upgrade::new(
                    "Gen 4, upgrade 1",
                    Float::with_val(20, 10),
                    MulitType::PointsMulti,
                    UpgradeType::Generator,
                    Float::with_val(20, 2),
                ),
                Upgrade::new(
                    "Gen 4, upgrade 2",
                    Float::with_val(20, 10),
                    MulitType::PointsMulti,
                    UpgradeType::Generator,
                    Float::with_val(20, 2),
                ),
            ],
            vec![
                Upgrade::new(
                    "Gen 5, upgrade 1",
                    Float::with_val(20, 10),
                    MulitType::PointsMulti,
                    UpgradeType::Generator,
                    Float::with_val(20, 2),
                ),
                Upgrade::new(
                    "Gen 5, upgrade 2",
                    Float::with_val(20, 10),
                    MulitType::PointsMulti,
                    UpgradeType::Generator,
                    Float::with_val(20, 2),
                ),
            ],
            vec![
                Upgrade::new(
                    "Gen 6, upgrade 1",
                    Float::with_val(20, 10),
                    MulitType::PointsMulti,
                    UpgradeType::Generator,
                    Float::with_val(20, 2),
                ),
                Upgrade::new(
                    "Gen 6, upgrade 2",
                    Float::with_val(20, 10),
                    MulitType::PointsMulti,
                    UpgradeType::Generator,
                    Float::with_val(20, 2),
                ),
            ],
            vec![
                Upgrade::new(
                    "Gen 7, upgrade 2",
                    Float::with_val(20, 10),
                    MulitType::PointsMulti,
                    UpgradeType::Generator,
                    Float::with_val(20, 2),
                ),
                Upgrade::new(
                    "Gen 7, upgrade 2",
                    Float::with_val(20, 10),
                    MulitType::PointsMulti,
                    UpgradeType::Generator,
                    Float::with_val(20, 2),
                ),
            ],
            vec![
                Upgrade::new(
                    "Gen 8, upgrade 1",
                    Float::with_val(20, 10),
                    MulitType::PointsMulti,
                    UpgradeType::Generator,
                    Float::with_val(20, 2),
                ),
                Upgrade::new(
                    "Gen 8, upgrade 2",
                    Float::with_val(20, 10),
                    MulitType::PointsMulti,
                    UpgradeType::Generator,
                    Float::with_val(20, 2),
                ),
            ],
            vec![
                Upgrade::new(
                    "Gen 9, upgrade 1",
                    Float::with_val(20, 10),
                    MulitType::PointsMulti,
                    UpgradeType::Generator,
                    Float::with_val(20, 2),
                ),
                Upgrade::new(
                    "Gen 9, upgrade 2",
                    Float::with_val(20, 10),
                    MulitType::PointsMulti,
                    UpgradeType::Generator,
                    Float::with_val(20, 2),
                ),
            ],
            vec![
                Upgrade::new(
                    "Gen 10, upgrade 1",
                    Float::with_val(20, 10),
                    MulitType::PointsMulti,
                    UpgradeType::Generator,
                    Float::with_val(20, 2),
                ),
                Upgrade::new(
                    "Gen 10, upgrade 2",
                    Float::with_val(20, 10),
                    MulitType::PointsMulti,
                    UpgradeType::Generator,
                    Float::with_val(20, 2),
                ),
            ],
            vec![
                Upgrade::new(
                    "Gen 11, upgrade 1",
                    Float::with_val(20, 10),
                    MulitType::PointsMulti,
                    UpgradeType::Generator,
                    Float::with_val(20, 2),
                ),
                Upgrade::new(
                    "Gen 11, upgrade 2",
                    Float::with_val(20, 10),
                    MulitType::PointsMulti,
                    UpgradeType::Generator,
                    Float::with_val(20, 2),
                ),
            ],
            vec![
                Upgrade::new(
                    "Gen 12, upgrade 1",
                    Float::with_val(20, 10),
                    MulitType::PointsMulti,
                    UpgradeType::Generator,
                    Float::with_val(20, 2),
                ),
                Upgrade::new(
                    "Gen 12, upgrade 2",
                    Float::with_val(20, 10),
                    MulitType::PointsMulti,
                    UpgradeType::Generator,
                    Float::with_val(20, 2),
                ),
            ],
            vec![
                Upgrade::new(
                    "Gen 13, upgrade 1",
                    Float::with_val(20, 10),
                    MulitType::PointsMulti,
                    UpgradeType::Generator,
                    Float::with_val(20, 2),
                ),
                Upgrade::new(
                    "Gen 13, upgrade 2",
                    Float::with_val(20, 10),
                    MulitType::PointsMulti,
                    UpgradeType::Generator,
                    Float::with_val(20, 2),
                ),
            ],
        ];
        generator_vec_of_vecs
    }
}
