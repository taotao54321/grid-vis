mod font;
pub mod image;

pub trait GridVis {
    fn col_count(&self) -> usize;
    fn row_count(&self) -> usize;

    fn line_color(&self) -> [u8; 3];

    fn cell_width(&self) -> u32;
    fn cell_height(&self) -> u32;
    fn font_size(&self) -> u32;

    fn cell_color(&self, x: usize, y: usize) -> [u8; 3];
    fn cell_text(&self, x: usize, y: usize) -> Option<(String, [u8; 3])>;
}
