use grid_vis::GridVis;

#[derive(Debug)]
struct Grid;

impl Grid {
    fn new() -> Self {
        Self
    }
}

impl GridVis for Grid {
    fn col_count(&self) -> usize {
        8
    }
    fn row_count(&self) -> usize {
        6
    }

    fn line_color(&self) -> [u8; 3] {
        [0x00, 0x00, 0x00]
    }

    fn cell_width(&self) -> u32 {
        32
    }
    fn cell_height(&self) -> u32 {
        24
    }
    fn font_size(&self) -> u32 {
        20
    }

    fn cell_color(&self, x: usize, y: usize) -> [u8; 3] {
        const COLORS: &[[u8; 3]] = &[
            [0xFF, 0x00, 0x00],
            [0x00, 0xFF, 0x00],
            [0xFF, 0xFF, 0x00],
            [0x00, 0x80, 0xFF],
            [0xFF, 0x00, 0xFF],
            [0x00, 0xFF, 0xFF],
        ];

        let i = (x + y) % COLORS.len();
        COLORS[i]
    }

    fn cell_text(&self, x: usize, _y: usize) -> Option<(String, [u8; 3])> {
        match x % 4 {
            1 => Some(("@".to_owned(), [0x00, 0x00, 0x00])),
            _ => None,
        }
    }
}

fn main() -> eyre::Result<()> {
    let grid = Grid::new();

    let img = grid_vis::image::visualize(&grid);
    img.save("image.png")?;

    Ok(())
}
