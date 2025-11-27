static INPUT: &str = include_str!("../input/dayone.txt");

fn split_string(string: &str) -> (&str, &str) {
    return match string.split_once("   ") {
        Some((a, b)) => (a, b),
        None => (string, ""), 
    }
}

fn int_diff((i1, i2): (&i32, &i32)) -> i32 {
    return (i1 - i2).abs();
}

fn read_lines() {
    let split_lines = INPUT.lines().map(split_string);
    let mut first_locations: Vec<i32> = split_lines
        .clone()
        .map(|x| x.0.parse::<i32>().unwrap()).collect();
    let mut second_locations: Vec<i32> = split_lines
        .clone()
        .map(|x| x.1.parse::<i32>().unwrap())
        .collect();

    first_locations.sort();
    second_locations.sort();

    let diff_list= first_locations.iter().zip(second_locations.iter()).map(int_diff);
    let total: i32 = diff_list.sum();
    println!("Sum : {}", total)
}


fn main() {
    read_lines();
}