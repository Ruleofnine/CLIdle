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
        app.debug_info.insert_and_goto_bottom(format!("upgrade index {:?}",app.upgrade_state.upgrade_index));
        app.debug_info.insert_and_goto_bottom(format!("upgrade list len {:?}",app.upgrade_state.upgrade_index.len()));
        app.debug_info.insert_and_goto_bottom(format!("upgrade_state {:?}",app.upgrade_state.state));
        app.debug_info.insert_and_goto_bottom(format!("max index {:?}",app.upgrade_state.max_index));
        // app.debug_info.insert_and_goto_bottom(format!("{:?}",app.))
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
