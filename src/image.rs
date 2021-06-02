use image::{Rgba, RgbaImage};

use crate::font::FONT;
use crate::GridVis;

pub fn visualize<G: GridVis>(grid: &G) -> RgbaImage {
    let mut img = new_image(grid);

    draw(&mut img, grid);

    img
}

fn draw<G: GridVis>(img: &mut RgbaImage, grid: &G) {
    draw_lines(img, grid);
    draw_cells(img, grid);
}

fn draw_cells<G: GridVis>(img: &mut RgbaImage, grid: &G) {
    let cw = grid.cell_width();
    let ch = grid.cell_height();
    let font_scale = new_font_scale(grid.font_size());

    for (row, col) in itertools::iproduct!(0..grid.row_count(), 0..grid.col_count()) {
        let x = 1 + (col as u32) * (cw + 1);
        let y = 1 + (row as u32) * (ch + 1);

        let rect = imageproc::rect::Rect::at(x as i32, y as i32).of_size(cw, ch);
        let color_bg = new_rgba(grid.cell_color(col, row));
        imageproc::drawing::draw_filled_rect_mut(img, rect, color_bg);

        // TODO: テキストのセンタリング
        if let Some((text, rgb)) = grid.cell_text(col, row) {
            let color_text = new_rgba(rgb);
            imageproc::drawing::draw_text_mut(img, color_text, x, y, font_scale, &FONT, &text);
        }
    }
}

fn draw_lines<G: GridVis>(img: &mut RgbaImage, grid: &G) {
    let cw = grid.cell_width();
    let ch = grid.cell_height();
    let color = new_rgba(grid.line_color());

    for x in (0..img.width()).step_by((cw + 1) as usize) {
        let p1 = (x as f32, 0.0);
        let p2 = (x as f32, img.height() as f32);
        imageproc::drawing::draw_line_segment_mut(img, p1, p2, color);
    }

    for y in (0..img.height()).step_by((ch + 1) as usize) {
        let p1 = (0.0, y as f32);
        let p2 = (img.width() as f32, y as f32);
        imageproc::drawing::draw_line_segment_mut(img, p1, p2, color);
    }
}

fn new_image<G: GridVis>(grid: &G) -> RgbaImage {
    let nc = grid.col_count() as u32;
    let nr = grid.row_count() as u32;
    let cw = grid.cell_width();
    let ch = grid.cell_height();

    // グリッド線を含めて計算。
    let w = nc * cw + nc + 1;
    let h = nr * ch + nr + 1;

    RgbaImage::new(w, h)
}

fn new_rgba(rgb: [u8; 3]) -> Rgba<u8> {
    Rgba([rgb[0], rgb[1], rgb[2], 0xFF])
}

fn new_font_scale(px: u32) -> rusttype::Scale {
    let px = px as f32;
    rusttype::Scale { x: px, y: px }
}
