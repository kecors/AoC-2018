use std::io::{stdin, Read};

fn solve(serial_number: i32, min_dial: usize, max_dial: usize) -> ((usize, usize, usize)) {
    let mut grid = Vec::new();

    for y0 in 0..300 {
        let mut row = Vec::new();
        for x0 in 0..300 {
            let rack_id = (x0 as i32 + 1) + 10;
            let mut power_level = rack_id * (y0 as i32 + 1);
            power_level += serial_number;
            power_level *= rack_id;
            power_level = (power_level / 100) % 10;
            power_level -= 5;
            row.push(power_level);
        }
        grid.push(row);
    }

    let mut memoized_total_power = vec![vec![None; 300]; 300];
    let mut largest_total_power = i32::min_value();
    let mut solution = None;

    for dial in min_dial..=max_dial {
        for y0 in 0..=(300 - dial) {
            for x0 in 0..=(300 - dial) {
                let mut total_power = 0;
                if let Some(memo) = memoized_total_power[y0][x0] {
                    total_power += memo;
                    for yn in 0..(dial - 1) {
                        total_power += grid[y0 + yn][x0 + dial - 1];
                    }
                    for xn in 0..dial {
                        total_power += grid[y0 + dial - 1][x0 + xn];
                    }
                } else {
                    for yn in 0..dial {
                        for xn in 0..dial {
                            total_power += grid[y0 + yn][x0 + xn];
                        }
                    }
                }
                memoized_total_power[y0][x0] = Some(total_power);

                if total_power > largest_total_power {
                    largest_total_power = total_power;
                    solution = Some((x0 + 1, y0 + 1, dial));
                }
            }
        }
    }

    solution.unwrap()
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let serial_number: i32 = input.trim().parse().unwrap();

    let (x, y, _dial) = solve(serial_number, 3, 3);
    println!("Part 1: the X,Y coordinate of the top-left fuel cell of the 3x3 square with the largest total power is {},{}", x, y);

    let (x, y, dial) = solve(serial_number, 1, 300);
    println!(
        "Part 2: the X,Y,size identifier of the square with the largest total power is {},{},{}",
        x, y, dial
    );
}
