fn main() {
    let input = include_str!("../input_rr");
    let (left, right) = input
        .lines()
        .map(|x| {
            let mut x = x.split_ascii_whitespace();
            (
                x.next().unwrap().parse::<i32>().unwrap(),
                x.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect::<(Vec<_>, Vec<_>)>();

    // let part1 = part1(left, right);
    // dbg!(&part1);
    let part2 = part2(left, right);
    dbg!(&part2);
}

fn part1(mut left: Vec<i32>, mut right: Vec<i32>) -> i32 {
    left.sort();
    right.sort();

    left.into_iter()
        .zip(right)
        .map(|(x, y)| (y - x).abs())
        .sum()
}

fn part2(left: Vec<i32>, right: Vec<i32>) -> i32 {
    left.iter()
        .map(|x| right.iter().filter(|y| x == *y).count() as i32 * x)
        .sum()
}
