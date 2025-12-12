use helpers::*;

mod structs;
mod tests;

use structs::*;

aoc_main!();

fn part_1(input: &[String]) -> i64 {
    let puzzle_data = parse_puzzledata(input);
    
    puzzle_data.regions.iter().map(|region| {
        easy_fit_check(region, &puzzle_data.shapes)
    }).filter(|&result| result).count() as i64

    // puzzle_data
    //     .regions
    //     .iter()
    //     .map(|region| {
    //         if check_fit(region, &puzzle_data.shapes) {
    //             1
    //         } else {
    //             0
    //         }
    //     })
    //     .sum()
}

fn part_2(input: &[String]) -> i64 {
    todo!();
}

fn parse_puzzledata(input: &[String]) -> PuzzleData {
    let mut iterator = input.iter();

    let mut shapes: Vec<Shape> = Vec::new();
    let mut regions: Vec<Region> = Vec::new();

    while let Some(line) = iterator.next() {
        let parts = line.split(":").collect::<Vec<_>>();

        if parts[1] == "" {
            // Next couple of lines will be a shape definition
            let shape = iterator
                .by_ref()
                .take_while(|line| !line.is_empty())
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '#' => true,
                            '.' => false,
                            _ => unreachable!("Invalid present shape character"),
                        })
                        .collect::<Vec<bool>>()
                })
                .collect::<Vec<Vec<bool>>>();
            shapes.push(Shape(shape));
        } else {
            // The second part contains the region definition
            let dimensions: (usize, usize) = parts[0]
                .split_once('x')
                .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                .unwrap();
            let requirements: Vec<usize> = parts[1]
                .trim()
                .split(' ')
                .map(|s| s.parse().unwrap())
                .collect();
            regions.push(Region {
                dimensions,
                requirements,
            });
        }
    }

    PuzzleData { shapes, regions }
}

fn easy_fit_check(region: &Region, shapes: &[Shape]) -> bool {
    // Check if there's enough area to even service all shapes assuming we can
    // split shapes
    let region_area = region.dimensions.0 * region.dimensions.1;
    let min_shape_required_area = region.requirements.iter().enumerate().map(|(i, &req)| {
        let shape = &shapes[i];
        shape.area() * req
    }).sum::<usize>();

    if region_area < min_shape_required_area {
        return false;
    }

    // Check if each 3x3 entry can fit in the region
    let region_allowed_entries = region.dimensions.0 / 3 * region.dimensions.1 / 3;
    let total_entries = region.requirements.iter().sum::<usize>();

    if total_entries < region_allowed_entries {
        return true;
    }

    true
}

// /// Check if a region can be filled with the given shapes
// fn check_fit(region: &Region, shapes: &[Shape]) -> bool {
//     let mut shapes_left_queue = Vec::new();

//     // Put all shape indices in queue
//     for (index, entry) in region.requirements.iter().enumerate() {
//         for _ in 0..*entry {
//             shapes_left_queue.push(index);
//         }
//     }

//     let mut placements: Vec<Placement> = Vec::new();

//     // Generate 2D map of region
//     let mut map = vec![vec![false; region.dimensions.0]; region.dimensions.1];

//     // Until we run out of items in the queue, try stuffing them into the map
//     while let Some(shape_index) = shapes_left_queue.pop() {
//         let shape = &shapes[shape_index];

//         // Try placing shape on map
//         let result = place_shape_on_map_next_fit(&mut map, shape, shape_index, None);

//         if let Some(placement) = result {
//             placements.push(placement);
//             continue;
//         }

//         // If shape does not fit, push back into queue, pop the last placement,
//         // and see if it can be fit
//         shapes_left_queue.push(shape_index);
//         let mut backtrack_found = false;
//         while let Some(last_placement) = placements.pop() {
//             let last_shape = &shapes[last_placement.shape_index];

//             // Undo placement on map
//             undo_place_shape_on_map(
//                 &mut map,
//                 last_shape,
//                 last_placement.x,
//                 last_placement.y,
//                 &last_placement.rotation,
//             );

//             // Try placing shape on next possible position
//             let result = place_shape_on_map_next_fit(
//                 &mut map,
//                 last_shape,
//                 last_placement.shape_index,
//                 Some(last_placement),
//             );

//             // If we were able to place the previous shape in another position,
//             // we can now continue trying to fit other shapes
//             if let Some(placement) = result {
//                 placements.push(placement);
//                 backtrack_found = true;
//                 break;
//             } else {
//                 // The last placement also did not fit, backtrack even more
//                 shapes_left_queue.push(last_placement.shape_index);
//             }
//         }

//         if !backtrack_found {
//             // This region cannot support the requirements
//             return false;
//         }
//     }

//     if shapes_left_queue.is_empty() {
//         return true;
//     }
//     false
// }

// // Checks to see if we can place the given shape on the current map. If so, place
// // it and return metadata on where it was placed. If not, return None.
// fn place_shape_on_map_next_fit(
//     map: &mut Vec<Vec<bool>>,
//     shape: &Shape,
//     shape_index: usize,
//     previous_placement: Option<Placement>,
// ) -> Option<Placement> {
//     let mut current_x = 0;
//     let mut current_y = 0;
//     let mut current_rotation = Rotation::None;

//     if let Some(previous_placement) = previous_placement {
//         current_x = previous_placement.x;
//         current_y = previous_placement.y;
//         current_rotation = previous_placement.rotation;

//         // Try next rotation and/or placement
//         if current_rotation == Rotation::R270 {
//             match current_x < map[0].len() - shape.width() - 1 {
//                 true => {
//                     current_x += 1;
//                 }
//                 false => {
//                     current_x = 0;
//                     current_y += 1;
//                 }
//             }
//         }
//         current_rotation = current_rotation.next();
//     }

//     // Start iterating
//     while current_y <= map.len() - shape.height() {
//         while current_x <= map[current_y].len() - shape.width() {
//             // See if shape will fit here with the four rotations
//             loop {
//                 if shape_fits_map(&map, shape, current_x, current_y, &current_rotation) {
//                     place_shape_on_map(map, shape, current_x, current_y, &current_rotation);
//                     return Some(Placement {
//                         x: current_x,
//                         y: current_y,
//                         rotation: current_rotation,
//                         shape_index: shape_index,
//                     });
//                 }

//                 current_rotation = current_rotation.next();
//                 if current_rotation == Rotation::None {
//                     break;
//                 }
//             }
//             // All four rotations failed to be placed, try next spot
//             current_x += 1;
//         }
//         current_x = 0;
//         current_y += 1;
//     }

//     // This shape can't be placed
//     None
// }

// /// Checks if the given shape fits the given map with the given position and rotation.
// /// We assume we will NOT go off bounds with the inputs given, and the function
// /// will panic otherwise.
// fn shape_fits_map(
//     map: &[Vec<bool>],
//     shape: &Shape,
//     current_x: usize,
//     current_y: usize,
//     current_rotation: &Rotation,
// ) -> bool {
//     match current_rotation {
//         Rotation::None => {
//             for (y, row) in shape.0.iter().enumerate() {
//                 for (x, &cell) in row.iter().enumerate() {
//                     if cell && map[current_y + y][current_x + x] {
//                         return false;
//                     }
//                 }
//             }
//         }
//         Rotation::R90 => {
//             for (y, row) in shape.0.iter().rev().enumerate() {
//                 for (x, &cell) in row.iter().enumerate() {
//                     if cell && map[current_y + y][current_x + x] {
//                         return false;
//                     }
//                 }
//             }
//         }
//         Rotation::R180 => {
//             for (y, row) in shape.0.iter().rev().enumerate() {
//                 for (x, &cell) in row.iter().rev().enumerate() {
//                     if cell && map[current_y + y][current_x + x] {
//                         return false;
//                     }
//                 }
//             }
//         }
//         Rotation::R270 => {
//             for (y, row) in shape.0.iter().enumerate() {
//                 for (x, &cell) in row.iter().rev().enumerate() {
//                     if cell && map[current_y + y][current_x + x] {
//                         return false;
//                     }
//                 }
//             }
//         }
//     }

//     true
// }

// fn place_shape_on_map(
//     map: &mut Vec<Vec<bool>>,
//     shape: &Shape,
//     start_x: usize,
//     start_y: usize,
//     rotation: &Rotation,
// ) {
//     match rotation {
//         Rotation::None => {
//             for (y, row) in shape.0.iter().enumerate() {
//                 for (x, &cell) in row.iter().enumerate() {
//                     if cell {
//                         map[start_y + y][start_x + x] = true;
//                     }
//                 }
//             }
//         }
//         Rotation::R90 => {
//             for (y, row) in shape.0.iter().rev().enumerate() {
//                 for (x, &cell) in row.iter().enumerate() {
//                     if cell {
//                         map[start_y + y][start_x + x] = true;
//                     }
//                 }
//             }
//         }
//         Rotation::R180 => {
//             for (y, row) in shape.0.iter().rev().enumerate() {
//                 for (x, &cell) in row.iter().rev().enumerate() {
//                     if cell {
//                         map[start_y + y][start_x + x] = true;
//                     }
//                 }
//             }
//         }
//         Rotation::R270 => {
//             for (y, row) in shape.0.iter().enumerate() {
//                 for (x, &cell) in row.iter().rev().enumerate() {
//                     if cell {
//                         map[start_y + y][start_x + x] = true;
//                     }
//                 }
//             }
//         }
//     }
// }

// fn undo_place_shape_on_map(
//     map: &mut Vec<Vec<bool>>,
//     shape: &Shape,
//     start_x: usize,
//     start_y: usize,
//     rotation: &Rotation,
// ) {
//     match rotation {
//         Rotation::None => {
//             for (y, row) in shape.0.iter().enumerate() {
//                 for (x, &cell) in row.iter().enumerate() {
//                     if cell {
//                         map[start_y + y][start_x + x] = false;
//                     }
//                 }
//             }
//         }
//         Rotation::R90 => {
//             for (y, row) in shape.0.iter().rev().enumerate() {
//                 for (x, &cell) in row.iter().enumerate() {
//                     if cell {
//                         map[start_y + y][start_x + x] = false;
//                     }
//                 }
//             }
//         }
//         Rotation::R180 => {
//             for (y, row) in shape.0.iter().rev().enumerate() {
//                 for (x, &cell) in row.iter().rev().enumerate() {
//                     if cell {
//                         map[start_y + y][start_x + x] = false;
//                     }
//                 }
//             }
//         }
//         Rotation::R270 => {
//             for (y, row) in shape.0.iter().enumerate() {
//                 for (x, &cell) in row.iter().rev().enumerate() {
//                     if cell {
//                         map[start_y + y][start_x + x] = false;
//                     }
//                 }
//             }
//         }
//     }
// }
