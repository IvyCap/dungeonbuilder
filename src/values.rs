pub enum Style {
    Cairn,
    Knave,
}

pub struct DungeonStyle{
    style: Style,
    room_type: Vec<(u32, String)>,

}

