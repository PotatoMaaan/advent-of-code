fn main() {
    let input = include_str!("../input_test");
    let input = input
        .lines()
        .map(|x| x.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut cnt = 0;

    let mut up = 0;
    let mut down = 0;
    let mut left = 0;
    let mut right = 0;
    let mut top_left = 0;
    let mut top_right = 0;
    let mut bottom_left = 0;
    let mut bottom_right = 0;

    for (i, x) in input.iter().enumerate() {
        for (j, _) in x.iter().enumerate() {
            //right
            if let Some(found) = input.get(i).and_then(|x| x.get(j..j + 4)) {
                assert!(found.len() <= 4, "right: {}", found.len());

                if found == b"XMAS" {
                    cnt += 1;
                    right += 1;
                    //pp(result);
                }
            }

            //left
            if let Some(found) = input.get(i).and_then(|x| x.get(j.saturating_sub(3)..=j)) {
                assert!(found.len() <= 4, "left: {}", found.len());

                if found == b"XMAS" {
                    cnt += 1;
                    left += 1;
                    //pp(result);
                }
            }

            //up
            if let Some(found) = (0..4)
                .map(|offset| {
                    input
                        .get(i.saturating_sub(3 - offset))
                        .and_then(|row| row.get(j))
                })
                .map(|x| x.copied())
                .collect::<Option<Vec<_>>>()
            {
                if found == b"XMAS" {
                    cnt += 1;
                    up += 1;
                }
            }

            //down
            if let Some(found) = (0..4)
                .map(|offset| input.get(i + offset).and_then(|row| row.get(j)))
                .map(|x| x.copied())
                .collect::<Option<Vec<_>>>()
            {
                if found == b"XMAS" {
                    cnt += 1;
                    down += 1;
                }
            }

            //bottom right
            let found = (0..b"XMAS".len())
                .map(|offset| (offset, input.get(i + offset)))
                .map(|(offset, x)| x.map(|x| x.get(j + offset)))
                .flatten()
                .flatten()
                .copied()
                .collect::<Vec<_>>();
            if found == b"XMAS" {
                cnt += 1;
                bottom_right += 1;
                pp(&found);
            }

            //bottom left
            let found = (0..b"XMAS".len())
                .map(|offset| (offset, input.get(i + offset)))
                .map(|(offset, x)| x.map(|x| x.get(j.saturating_sub(offset))))
                .flatten()
                .flatten()
                .copied()
                .collect::<Vec<_>>();
            if found == b"XMAS" {
                cnt += 1;
                bottom_left += 1;
                pp(&found);
            }

            //top right
            let found = (0..b"XMAS".len())
                .map(|offset| (offset, input.get(i.saturating_sub(offset))))
                .map(|(offset, x)| x.map(|x| x.get(j + offset)))
                .flatten()
                .flatten()
                .copied()
                .collect::<Vec<_>>();
            if found == b"XMAS" {
                cnt += 1;
                top_right += 1;
                pp(&found);
            }

            //top left
            let found = (0..b"XMAS".len())
                .map(|offset| (offset, input.get(i.saturating_sub(offset))))
                .map(|(offset, x)| x.map(|x| x.get(j.saturating_sub(offset))))
                .flatten()
                .flatten()
                .copied()
                .collect::<Vec<_>>();
            if found == b"XMAS" {
                cnt += 1;
                top_left += 1;
            }
        }
    }

    dbg!(&up);
    dbg!(&down);
    dbg!(&left);
    dbg!(&right);
    dbg!(&top_left);
    dbg!(&top_right);
    dbg!(&bottom_left);
    dbg!(&bottom_right);

    dbg!(&cnt);
}

fn pp(x: &[u8]) {
    for x in x {
        print!(
            "{}",
            match *x as char {
                '.' => '_',
                rest => rest,
            }
        );
    }
    println!();
}
