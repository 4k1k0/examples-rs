use crate::identicon::grid;

pub type Point = (usize, usize);
pub type PixelMap = (Point, Point);

pub fn new(points: &Vec<grid::Point>) -> Vec<PixelMap> {
    points.iter().map(new_pixel_map).collect()
}

fn new_pixel_map((_x, index): &grid::Point) -> PixelMap {
    let horizontal = (index % 5) * 50;
    let vertical = (index / 5) * 50;
    let top_left: Point = (horizontal, vertical);
    let bottom_right: Point = (horizontal + 50, vertical + 50);
    (top_left, bottom_right)
}
