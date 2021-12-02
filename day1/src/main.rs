use std::fs;

fn main() {
    let num_vec: Vec<u32> = fs::read_to_string("input/input.txt")
        .expect("Failed to read file!")
        .lines()
        .map(|e| e.parse().expect("Failed to parse number to u32"))
        .collect();

    let part1 = &num_vec[..]
        .windows(2)
        .filter(|e| {
            let prev = e[0];
            let curr = e[1];

            curr > prev
        })
        .count();

    let window_by_3: &Vec<&[u32]> = &num_vec[..].windows(3).collect();
    let window_3_by_2 = window_by_3[..].windows(2);
    let part2 = window_3_by_2
        .filter(|e| {
            let prev_sum: u32 = e[0].iter().sum();
            let curr_sum: u32 = e[1].iter().sum();

            curr_sum > prev_sum
        })
        .count();

    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}
