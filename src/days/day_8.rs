use crate::utils;

pub fn exec() {
    // Read the input
    let data = utils::read_input("8");
    let data = data.trim_end().split("\n");

    // Setup result vars
    let (mut part_a, mut part_b) = (0u32, 0u32);

    // Load tree data
    let mut trees: Vec<Vec<u32>> = Vec::new();

    for line in data {
        trees.push(
            line.chars()
                .map(|c| c.to_string().parse::<u32>().unwrap())
                .collect(),
        )
    }

    // Get the size of the grid
    let x_max = trees.len();
    let y_max = trees[0].len();

    // Calculate the visibility of l-r and t-b
    let mut visibility_lr: Vec<Vec<bool>> = Vec::new();
    let mut visibility_tb: Vec<Vec<bool>> = Vec::new();

    let mut max_y = vec![0; trees.len()];

    for (y, row) in trees.iter().enumerate() {
        let mut max = 0u32;
        let mut row_visibility: Vec<bool> = vec![true; row.len()];
        let mut column_visibility: Vec<bool> = vec![true; trees.len()];
        for (x, &tree) in row.iter().enumerate() {
            if x == 0 {
                max = tree;
            } else if tree > max {
                max = tree;
            } else {
                row_visibility[x] = false;
            }

            if y == 0 {
                max_y[x] = tree;
            } else if tree > max_y[x] {
                max_y[x] = tree;
            } else {
                column_visibility[x] = false;
            }
        }
        visibility_lr.push(row_visibility.clone());
        visibility_tb.push(column_visibility.clone());
    }

    // Calculate the visibility r-l b-t
    // Do this by iterating in reverse
    let mut visibility_rl: Vec<Vec<bool>> = Vec::new();
    let mut visibility_bt: Vec<Vec<bool>> = Vec::new();

    let mut max_y = vec![0; trees.len()];

    for (y, row) in trees.iter().rev().enumerate() {
        let mut max = 0u32;
        let mut row_visibility: Vec<bool> = vec![true; row.len()];
        let mut column_visibility: Vec<bool> = vec![true; trees.len()];
        for (x, &tree) in row.iter().rev().enumerate() {
            if x == 0 {
                max = tree;
            } else if tree > max {
                max = tree;
            } else {
                row_visibility[x] = false;
            }

            if y == 0 {
                max_y[x] = tree;
            } else if tree > max_y[x] {
                max_y[x] = tree;
            } else {
                column_visibility[x] = false;
            }
        }
        row_visibility.reverse();
        visibility_rl.push(row_visibility.clone());
        visibility_bt.push(column_visibility.clone());
    }

    for row in visibility_bt.iter_mut() {
        row.reverse();
    }

    visibility_bt.reverse();
    visibility_rl.reverse();

    // Iterate over the result tables and calculate part a
    let all = [visibility_lr, visibility_rl, visibility_tb, visibility_bt];

    for x in 0..x_max {
        for y in 0..y_max {
            let visible = all.iter().any(|map| map[x][y]);
            part_a += if visible { 1 } else { 0 };
        }
    }

    // Transpose the trees so we can calculate the scenic score
    let transposed = transpose(&trees);

    // Find the most scenic tree
    for x in 0..x_max {
        for y in 0..y_max {
            let score = scenic_score(x, y, &trees, &transposed);

            if score > part_b {
                part_b = score;
            }
        }
    }

    println!("{part_a} {part_b}");
}

fn scenic_score(x: usize, y: usize, trees: &Vec<Vec<u32>>, transposed: &Vec<Vec<u32>>) -> u32 {
    let mut score = 1u32;

    let tree = &trees[y][x];

    let mut left_trees = trees[y][..x].iter().collect::<Vec<&u32>>();
    left_trees.reverse();

    let right_trees = trees[y][x + 1..].iter().collect::<Vec<&u32>>();

    let mut up_trees = transposed[x][..y].iter().collect::<Vec<&u32>>();
    up_trees.reverse();

    let down_trees = transposed[x][y + 1..].iter().collect::<Vec<&u32>>();

    let all = [left_trees, right_trees, up_trees, down_trees];

    for group in all {
        let mut inner = 0u32;
        for t in group {
            if t < tree {
                inner += 1;
            } else if t >= tree {
                inner += 1;
                break;
            }
        }
        score *= inner
    }

    score
}

fn transpose(matrix: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut transposed = vec![Vec::new(); matrix.len()];

    for row in matrix.iter() {
        for (index, item) in row.iter().enumerate() {
            transposed[index].push(*item);
        }
    }

    transposed
}
