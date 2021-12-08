fn main() {
    let input = include_str!("../input");

    part1(input);
    part2(input);
}

fn part1(input: &str) {
    println!("{}",
        input 
        .lines()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|line| line[1] > line[0])
        .count()
    );
}

fn part2(input: &str) {
    println!("{}",
        input 
        .lines()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>()
        .windows(3)
        .map(|y| y.iter().sum::<i32>())
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|line| line[1] > line[0])
        .count()
    );
}
