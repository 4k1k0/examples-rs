pub type Point = (u8, usize);

pub fn new(hex: [u8; 16]) -> Vec<Point> {
    let tmp_new_split = split(hex).map(mirror_row);
    let tmp_flaten = list_flaten(tmp_new_split);
    let tmp_index = with_index(tmp_flaten);

    filter_even_points(tmp_index)
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

