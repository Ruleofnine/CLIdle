use crate::{app::App, utils::format_number};
use std::collections::BTreeMap;
#[derive(Debug)]
pub enum InputMode {
    Normal,
    Editing,
}
pub fn take_input(input: &str, app: &mut App) {
    if input == "clear"{
        app.debug_info.items.clear();
    }
    if input == "test"{
        let upgrade_index = app.upgrade_state.upgrade_indexes[app.upgrade_state.state.selected().unwrap()];
        app.debug_info.insert_and_goto_bottom(format!("upgrade index {:?}",app.upgrade_state.upgrade_indexes));
        app.debug_info.insert_and_goto_bottom(format!("upgrade index selected {:?}",app.upgrade_state.upgrade_indexes[app.upgrade_state.state.selected().unwrap()]));
        app.debug_info.insert_and_goto_bottom(format!("upgrade list 0 {:?}",app.upgrade_list[upgrade_index.0][upgrade_index.1]));
        app.debug_info.insert_and_goto_bottom(format!("upgrade list len {:?}",app.upgrade_state.upgrade_indexes.len()));
        app.debug_info.insert_and_goto_bottom(format!("upgrade_state {:?}",app.upgrade_state.state));
        app.debug_info.insert_and_goto_bottom(format!("max index {:?}",app.upgrade_state.max_index));
        // app.debug_info.insert_and_goto_bottom(format!("{:?}",app.))
        // app.debug_info.insert_and_goto_bottom(format!("{:?}",app.tabs.tabs));
        // app.debug_info.insert_and_goto_bottom(format!("{:?}",app.tabs.index));
        // app.debug_info.insert_and_goto_bottom(format!("{:?}",app.tabs.tabtype));
        //
    }
    let float_map = BTreeMap::from([
        ("ppt", &app.player.ppt),
        ("pps",&app.player.pps),
        ("ppc",&app.player.ppc),
        ("prestige points",&app.player.prestige_points)
    ]);
    match float_map.get(input) {
        Some(i) => {
            app.output = format_number(*i);
        }
        _ => {app.output = String::from(input)}
    }
}
