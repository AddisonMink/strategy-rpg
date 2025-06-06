use super::*;

const DATA: UnitData = UnitData {
    name: ShortString::new("Goon"),
    glyph: Glyph::new('G', WHITE),
    side: Side::NPC,
    movement: 2,
    hp_max: 5,
};

pub fn make_goon(id: UnitId, coord: Coord) -> Unit {
    Unit::new_npc(
        id,
        coord,
        DATA,
        behavior::chase_nearest_player,
        behavior::select_action_noop,
    )
}
