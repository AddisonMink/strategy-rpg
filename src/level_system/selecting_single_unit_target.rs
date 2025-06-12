use macroquad::prelude::trace;

use crate::engine::*;
use crate::level_model::*;

pub fn update(level: &mut Level) {
    let LevelState::SelectingSingleUnitTarget {
        action,
        targets,
        selected_target,
    } = &mut level.state
    else {
        return;
    };

    let mouse_coord = grid::mouse_coord().filter(|c| targets.contains_key(c));
    if mouse_coord.is_some() && input::mouse_clicked() {
        let coord = mouse_coord.unwrap();
        trace!("Selected target at coord: {:?}", coord);
    } else if let Some(coord) = mouse_coord {
        *selected_target = Some(coord);
    }
}
