use crate::{
    generators::{BuyAmount, PointGenerator},
    input::{take_input, InputMode},
    player::Player,
    ui::StatefulList,
    upgrades::{Upgrade, UpgradeState},
};
#[derive(Clone, Copy, Debug)]
pub enum TabType {
    Main,
    Settings,
    Prestige,
    Debug,
}
pub struct TabsState<'a> {
    // pub titles: Vec<&'a str>,
    pub index: usize,
    pub tabtype: TabType,
    // pub tabtypes: Vec<TabType>,
    pub tabs: Vec<(&'a str, TabType)>,
}

impl<'a> TabsState<'a> {
    // pub fn new(titles: Vec<&'a str>, tabtypes: Vec<TabType>,tabs: BTreeMap<&'a str,TabType>) -> TabsState {
    pub fn new(tabs: Vec<(&'a str, TabType)>) -> TabsState {
        TabsState {
            // titles,
            index: 0,
            tabtype: TabType::Main,
            // tabtypes,
            tabs,
        }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.tabs.len();
        self.tabtype = self.tabs.iter().nth(self.index).unwrap().1;
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            // self.index = self.titles.len() - 1;
            self.index = (self.index + 1) % self.tabs.len();
        }
        // self.tabtype = self.tabtypes[self.index]
        self.tabtype = self.tabs.iter().nth(self.index).unwrap().1;
    }
}
pub struct App<'a> {
    pub title: &'a str,
    pub input: String,
    pub output: String,
    pub input_mode: InputMode,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
    pub enhanced_graphics: bool,
    pub player: Player,
    pub debug_info: StatefulList<String>,
    pub upgrade_list: Vec<Vec<Upgrade>>,
    pub upgrade_state: UpgradeState,
    pub unowned_pointgenerators: Vec<PointGenerator>,
}
impl<'a> App<'a> {
    pub fn new(
        title: &'a str,
        enhanced_graphics: bool,
        player: Player,
        unowned_pointgenerators: Vec<PointGenerator>,
        upgrade_list: Vec<Vec<Upgrade>>,
        // upgrade_indexes: Vec<(usize, usize)>,
    ) -> App<'a> {
        App {
            title,
            input: String::new(),
            output: String::new(),
            input_mode: InputMode::Normal,
            should_quit: false,
            player,
            tabs: TabsState::new(vec![
                ("Points", TabType::Main),
                // ("Settings", TabType::Settings),
                // ("Prestige", TabType::Prestige),
                ("Debug", TabType::Debug),
            ]),
            enhanced_graphics,
            debug_info: StatefulList::with_items(Vec::new()),
            upgrade_list,
            upgrade_state: UpgradeState::new(),
            unowned_pointgenerators,
        }
    }
    pub fn on_up(&mut self) {
        match self.tabs.tabtype {
            TabType::Main => {
                if self.player.owned_pointgenerators.state.selected().is_some() {
                    self.player.owned_pointgenerators.previous()
                } else {
                    self.upgrade_state.previous()
                }
            }
            TabType::Debug => self.debug_info.previous(),
            _ => {}
        }
    }

    pub fn on_down(&mut self) {
        match self.tabs.tabtype {
            TabType::Main => {
                if self.player.owned_pointgenerators.state.selected().is_some()
                    || !self.upgrade_state.state.selected().is_some()
                {
                    self.player.owned_pointgenerators.next()
                } else if true {
                    self.upgrade_state.next();
                }
            }
            TabType::Debug => {
                self.debug_info.next();
            }
            _ => {}
        }
    }
    pub fn on_left(&mut self) {
        match self.tabs.tabtype {
            TabType::Main => match self.upgrade_state.state.selected() {
                Some(_) => {
                    self.upgrade_state.unselect();
                    self.player.owned_pointgenerators.next();
                }
                None => {
                    self.player.owned_pointgenerators.unselect();
                    self.upgrade_state.state.select(Some(0))
                }
            },
            TabType::Debug => {}
            _ => {}
        }
    }
    pub fn on_right(&mut self) {
        match self.tabs.tabtype {
            TabType::Main => match self.upgrade_state.state.selected() {
                Some(_) => {
                    self.upgrade_state.unselect();
                    self.player.owned_pointgenerators.next();
                }
                None => {
                    self.player.owned_pointgenerators.unselect();
                    self.upgrade_state.state.select(Some(0))
                }
            },
            TabType::Debug => {}
            _ => {}
        }
    }
    pub fn on_esc(&mut self) {
        match self.tabs.tabtype {
            TabType::Main => {
                self.player.owned_pointgenerators.unselect();
                self.upgrade_state.unselect()
            }
            TabType::Debug => self.debug_info.unselect(),
            _ => {}
        }
    }

    pub fn on_tab(&mut self) {
        self.tabs.next();
    }

    pub fn on_backtab(&mut self) {
        self.tabs.previous();
    }

    pub fn on_key(&mut self, c: char) {
        match self.input_mode {
            InputMode::Normal => match c {
                'q' => {
                    self.should_quit = true;
                }
                'm' => match self.player.owned_pointgenerators.state.selected() {
                    Some(i) => self.player.buy_generator_amount(
                        i,
                        BuyAmount::MAX,
                        &mut self.unowned_pointgenerators,
                    ),
                    _ => {}
                },
                // '5' => match self.player.generators.state.selected() {
                //     Some(i) => self
                //         .player
                //         .buy_generator_amount(i, generators::BuyAmount::FIVE),
                //     _ => {}
                // },
                'i' => {
                    if matches!(self.input_mode, InputMode::Normal)
                        && matches!(self.tabs.tabtype, TabType::Debug)
                    {
                        self.input_mode = InputMode::Editing;
                    } else {
                        self.input_mode = InputMode::Normal
                    }
                }
                'd' => {
                    self.debug_info.insert_and_goto_bottom(format!(
                        "upgrade index {:?}",
                        self.upgrade_state.upgrade_indexes
                    ));
                    self.debug_info.insert_and_goto_bottom(format!(
                        "upgrade list len {:?}",
                        self.upgrade_state.upgrade_indexes.len()
                    ));
                    self.debug_info.insert_and_goto_bottom(format!(
                        "upgrade_state {:?}",
                        self.upgrade_state.state
                    ));
                    self.debug_info.insert_and_goto_bottom(format!(
                        "max index {:?}",
                        self.upgrade_state.max_index
                    ));
                }
                _ => {
                    self.player.click_points();
                }
            },
            InputMode::Editing => {}
        }
    }
    pub fn on_enter(&mut self) {
        match self.tabs.tabtype {
            TabType::Main => {
                // later on will need to see if upgrades are seleceted
                let point_generator_state = self.player.owned_pointgenerators.state.selected();
                let upgrade_list_state = self.upgrade_state.state.selected();
                if point_generator_state.is_some() {
                    self.player.buy_generator_amount(
                        point_generator_state.unwrap(),
                        BuyAmount::ONE,
                        &mut self.unowned_pointgenerators,
                    )
                } else if upgrade_list_state.is_some() && self.upgrade_state.max_index > 0 {
                    let bought = self.player.buy_upgrade(
                        &mut self.upgrade_list,
                        &mut self.upgrade_state.upgrade_indexes,
                        upgrade_list_state.unwrap(),
                    );
                    if bought {
                        self.upgrade_state.bought();
                    }
                }
            }
            TabType::Debug => match self.input_mode {
                InputMode::Editing => {
                    let command: String = self.input.drain(..).collect();
                    self.debug_info
                        .items
                        .insert(self.debug_info.items.len(), command.clone());
                    take_input(&command, self);
                }
                _ => {}
            },
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {
        // Update progress
        self.player.increase_points();
    }
}
