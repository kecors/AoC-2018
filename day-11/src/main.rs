use std::io::{stdin, Read};

fn solve(serial_number: i32) -> (usize, usize) {
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

    let mut largest_total_power = i32::min_value();
    let mut largest_coordinate = None;

    for y0 in 0..298 {
        for x0 in 0..298 {
            let mut total_power = 0;
            for yn in 0..3 {
                for xn in 0..3 {
                    total_power += grid[y0 + yn][x0 + xn];
                }
            }
            if total_power > largest_total_power {
                largest_total_power = total_power;
                largest_coordinate = Some((x0 + 1, y0 + 1));
            }
        }
    }

    largest_coordinate.unwrap()
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let serial_number: i32 = input.trim().parse().unwrap();
    println!("serial_number = {}", serial_number);

    let part1 = solve(serial_number);
    println!("Part 1: the X,Y coordinate of the top-left fuel cell of the 3x3 square with the largest total power is {:?}", part1);
}
