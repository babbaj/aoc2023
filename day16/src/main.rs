use std::collections::HashSet;

fn in_bounds(x: i32, y: i32, board: &[&[u8]]) -> bool {
    return y >= 0 && y < (board.len() as i32) && x >= 0 && x < (board[0].len() as i32);
}

fn simulate0(mut x: i32, mut y: i32, mut dx: i32, mut dy: i32, input: &[&[u8]], visited: &mut HashSet<(i32, i32)>, reflections: &mut HashSet<(i32, i32)>) {
    while in_bounds(x, y, input) {
        if dx != 0 && dy != 0 {
            panic!("dx and dy != 0");
        }
        if dx == 0 && dy == 0 {
            panic!("not moving");
        }
        let c = input[y as usize][x as usize] as char;
        visited.insert((x, y));
        match c {
            '.' => {},
            '\\' => {
                std::mem::swap(&mut dx, &mut dy);
            },
            '/' => {
                dx *= -1;
                dy *= -1;
                std::mem::swap(&mut dx, &mut dy);
            }
            '-' => {
                // return if already reflected
                if dy != 0 {
                    if !reflections.insert((x, y)) {
                        return;
                    }
                    simulate0(x - 1, y, -1, 0, input, visited, reflections);
                    simulate0(x + 1, y, 1, 0, input, visited, reflections);
                    return;
                }
            },
            '|' => {
                if dx != 0 {
                    // return if already reflected
                    if !reflections.insert((x, y)) {
                        return;
                    }
                    simulate0(x, y - 1, 0, -1, input, visited, reflections);
                    simulate0(x , y + 1, 0, 1, input, visited, reflections);
                    return;
                }
            }
            _ => panic!("invalid char {c}")
        }
        x += dx;
        y += dy;
    }
}

fn simulate(x: i32, y: i32, dx: i32, dy: i32, input: &[&[u8]]) -> HashSet<(i32, i32)> {
    let mut reflections = HashSet::<(i32, i32)>::new();
    let mut visited = HashSet::<(i32, i32)>::new();
    simulate0(x, y, dx, dy, input, &mut visited, &mut reflections);
    return visited;
}

fn render(w: usize, h: usize, visited: &HashSet<(i32, i32)>) {
    for y in 0..h {
        for x in 0..w {
            print!("{}", if visited.contains(&(x as i32, y as i32)) { '#' } else { '.' });
        }
        println!();
    }
}

fn part1(input: &[&[u8]]) {
    let visited = simulate(0, 0, 1, 0, &input);
    render(input[0].len(), input.len(), &visited);
    println!("{}", visited.len());
}

fn start_states(w: i32, h: i32) -> impl Iterator<Item = ((i32, i32), (i32, i32))> {
    let horizontal = (0..w).flat_map(move |x| [((x, 0), (0, 1)), ((x, h - 1), (0, -1))].into_iter());
    let veritcal = (0..h).flat_map(move |y| [((0, y), (1, 0)), ((w - 1, y), (-1, 0))].into_iter());
    return horizontal.chain(veritcal);
}

fn part2(input: &[&[u8]]) {
    let max = start_states(input[0].len() as i32, input.len() as i32)
        .map(|((x, y), (dx, dy))| simulate(x, y, dx, dy, input).len())
        .max();
    println!("{}", max.unwrap());
}

fn main() {
    let input = include_str!("../input").trim_end()
        .split('\n')
        .map(|s| s.as_bytes())
        .collect::<Vec<_>>();
    part1(&input);
    part2(&input);
}
