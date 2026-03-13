pub fn new(hex: [u8; 16]) -> [Point;25] {
    let tmp_new_split = split(hex).map(mirror_row);
    let x = list_flaten(tmp_new_split);
    with_index(x)
}

fn split(hex: [u8; 16]) -> [[u8; 3]; 5] {
    let mut result = [[0u8; 3]; 5];
    for (i, chunk) in hex.chunks_exact(3).take(5).enumerate() {
        result[i].copy_from_slice(chunk);
    }

    result
}

fn mirror_row(row: [u8; 3]) -> [u8; 5] {
    let mut res = [0u8; 5];
    res[0] = row[0];
    res[1] = row[1];
    res[2] = row[2];
    res[3] = row[1];
    res[4] = row[0];

    res
}

fn list_flaten(rows: [[u8; 5]; 5]) -> [u8; 25] {
    let mut res = [0u8; 25];
    let flat_iter = rows.iter().flatten();
    for (i, &byte) in flat_iter.enumerate() {
        res[i] = byte;
    }

    res
}

pub type Point = (u8, usize);

fn with_index(list: [u8; 25]) -> [Point; 25] {
    let mut points: [Point; 25] = [(0, 0); 25];
    for (i, &element) in list.iter().enumerate() {
        points[i] = (element, i);
    }

    points
}

fn filter_even_points(points: [Point;25])  -> Vec<Point> {
    points.into_iter().filter(|(a, _b)| a %2 == 0).collect()
}

fn build_pixel_map(points: Vec<Point>) -> Vec<PixelMap> {
    points.iter().map(new_pixel_map).collect()
}

type Foo = (usize, usize);

type PixelMap = (Foo, Foo);

fn new_pixel_map((_x, index): &Point) -> PixelMap {
    let horizontal = (index %5) * 50;
    let vertical = (index /5) * 50;
    let top_left: Foo = (horizontal, vertical);
    let bottom_right: Foo = (horizontal + 50, vertical + 50);
    (top_left, bottom_right)
}
