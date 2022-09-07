use rug::Float;
use tui::widgets::ListState;
#[derive(Clone, Debug)]
pub enum MulitType {
    PointsMulti,
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
    pub upgrade_index: Vec<(usize, usize)>,
}

impl UpgradeState {
    pub fn new(upgrade_indexes: Vec<(usize, usize)>) -> UpgradeState {
        UpgradeState {
            state: ListState::default(),
            max_index: 0,
            upgrade_index: upgrade_indexes,
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
                if self.max_index == 0 || i==0{
                    0
                } else if i == self.max_index-1{
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
    pub fn test_upgrade(number: usize, cost: f64) -> Upgrade {
        Upgrade {
            name: String::from(format!("Test Upgrade {}", number)),
            cost: Float::with_val(20, cost),
            mulittype: MulitType::PointsMulti,
            upgradetype: UpgradeType::Generator,
            number: Float::with_val(20, 2),
        }
    }
    pub fn test_upgrade_vec() -> Vec<Upgrade> {
        vec![
            Upgrade::test_upgrade(1, 1e1),
            Upgrade::test_upgrade(2, 1e4),
            Upgrade::test_upgrade(3, 1e5),
        ]
    }
    pub fn create_upgrade_indexes(upgrades: &mut Vec<Vec<Upgrade>>) -> Vec<(usize, usize)> {
        let mut upgrade_indexes: Vec<(usize, usize)> = Vec::new();
        for (gi, g) in upgrades.iter().enumerate() {
            for (ui, _) in g.iter().enumerate() {
                upgrade_indexes.push((gi, ui))
            }
        }
        upgrade_indexes
    }
    pub fn make_generator_upgrades() -> Vec<Vec<Upgrade>> {
        let generator_vec_of_vecs = vec![
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
                    Float::with_val(20, 10),
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
