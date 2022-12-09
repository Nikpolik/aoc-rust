mod helpers;

type InputType = Vec<Vec<i32>>;
type SolutionType = Vec<Vec<bool>>;

fn parse_input(input_str: &str) -> InputType {
    input_str
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .map(|d| d as i32)
                .collect()
        })
        .collect()
}

#[derive(Debug)]
enum Side {
    LeftRight,
    TopBottom,
    RightLeft,
    BottomTop,
}

fn get_range(side: &Side, start: usize, end: usize) -> Box<dyn Iterator<Item = usize>> {
    match side {
        Side::LeftRight | Side::TopBottom => Box::new(start..end),
        Side::RightLeft | Side::BottomTop => Box::new((start..end).rev()),
    }
}

fn find_clearings(i: usize, j: usize, input: &InputType) -> i32 {
    let mut total = 1;
    let mut side_score = 0;
    // left
    for k in get_range(&Side::RightLeft, 0, j) {
        side_score += 1;
        if input[i][k] >= input[i][j] {
            break;
        }
    }
    total = total * side_score;

    // right
    side_score = 0;
    for k in get_range(&Side::LeftRight, j + 1, input.len()) {
        side_score += 1;
        if input[i][k] >= input[i][j] {
            break;
        }
    }
    total = total * side_score;

    // top
    side_score = 0;
    for k in get_range(&Side::BottomTop, 0, i) {
        side_score += 1;
        if input[k][j] >= input[i][j] {
            break;
        }
    }
    total = total * side_score;

    // bottom
    side_score = 0;
    for k in get_range(&Side::TopBottom, i + 1, input.len()) {
        side_score += 1;
        if input[k][j] >= input[i][j] {
            break;
        }
    }
    total = total * side_score;
    total
}

fn check_side(side: Side, input: &InputType, visibility_status: &mut SolutionType) {
    let range = get_range(&side, 0, input.len());

    for i in range {
        let mut largest_height = -1;
        let inner = get_range(&side, 0, input.len());

        for j in inner {
            let (first, second) = match side {
                Side::LeftRight | Side::RightLeft => (i, j),
                Side::TopBottom | Side::BottomTop => (j, i),
            };
            let current_row_height = input[first][second];
            if current_row_height > largest_height {
                visibility_status[first][second] = true;
                largest_height = current_row_height;
            }
        }
    }
}

fn main() {
    let input_str = include_str!("../inputs/2022/day8/input.txt");
    let input = parse_input(input_str);
    let mut visibility_status: SolutionType = input
        .iter()
        .map(|row| row.iter().map(|_| false).collect())
        .collect();
    // first lets do one side, left to right
    // scan items row by row and left to right
    check_side(Side::TopBottom, &input, &mut visibility_status);
    check_side(Side::BottomTop, &input, &mut visibility_status);
    check_side(Side::RightLeft, &input, &mut visibility_status);
    check_side(Side::LeftRight, &input, &mut visibility_status);

    let mut max = 0;
    for i in 0..visibility_status.len() {
        for j in 0..visibility_status.len() {
            if visibility_status[i][j] {
                let score = find_clearings(i, j, &input);
                if score > max {
                    max = score;
                }
            }
        }
    }
    println!("{}", max);
}
