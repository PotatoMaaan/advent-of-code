use std::time::Instant;

fn main() {
    let input = include_str!("../input_bb");
    let start = Instant::now();

    let reports = input
        .lines()
        .map(|x| {
            x.split_ascii_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    println!("parsing: {:?}", start.elapsed());

    let start = Instant::now();
    let part1 = part1(&reports);
    println!("part1: {:?}", start.elapsed());
    dbg!(&part1);

    let start = Instant::now();
    let part2 = part2(&reports);
    println!("part2: {:?}", start.elapsed());
    dbg!(&part2);
}

fn has_error(x: &Vec<u32>) -> bool {
    let mut desc = false;

    x.windows(2)
        .enumerate()
        .map(|(i, x)| {
            if i == 0 && x[0] < x[1] {
                desc = true;
            } else if desc && x[0] > x[1] {
                return None;
            } else if !desc && x[0] < x[1] {
                return None;
            }

            Some(x[0].abs_diff(x[1]))
        })
        .all(|x| match x {
            Some(x) => x <= 3 && x != 0,
            None => false,
        })
}

fn part1(reports: &[Vec<u32>]) -> usize {
    reports.iter().map(has_error).filter(|x| *x).count()
}

fn part2(reports: &[Vec<u32>]) -> usize {
    reports
        .iter()
        .map(|x| {
            (0..x.len())
                .map(|i| {
                    let mut with_removed = x.clone();
                    with_removed.remove(i);
                    has_error(&with_removed)
                })
                .filter(|x| *x)
                .any(|_| true)
        })
        .filter(|x| *x)
        .count()
}
