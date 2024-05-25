use hexx::Hex;

#[derive(Default)]
pub struct MultiselectData {
    pub total_mouse_delta: f32,
    pub previously_selected_tiles: Vec<Hex>,
}

impl MultiselectData {
    pub fn clear(&mut self) {
        self.total_mouse_delta = 0.0;
        self.previously_selected_tiles.clear();
    }
}
