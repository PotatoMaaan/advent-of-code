fn main() {
    let input = include_str!("../input_rr");
    let input = input.chars().collect::<Vec<_>>();

    let part1 = part1(Lexer::new(&input));
    dbg!(&part1);

    let part2 = part2(Lexer::new(&input));
    dbg!(&part2);
}

fn part1(mut lex: Lexer) -> u32 {
    let mut collector = Vec::new();

    'parser: while let Some(c) = lex.next() {
        lex.reset_peek();

        match (c, lex.peek::<3>()) {
            ('m', Some(['u', 'l', '('])) => {
                let Some((num1, next)) = lex.parse_num() else {
                    continue 'parser;
                };
                if next != ',' {
                    continue 'parser;
                }

                let Some((num2, next)) = lex.parse_num() else {
                    continue 'parser;
                };
                if next != ')' {
                    continue 'parser;
                }

                collector.push((num1, num2));
            }
            (_, None) => break,
            _ => {}
        }
    }

    collector.iter().map(|(a, b)| a * b).sum::<u32>()
}

fn part2(mut lex: Lexer) -> u32 {
    let mut collector = Vec::new();

    let mut enabled = true;
    'parser: while let Some(c) = lex.next() {
        lex.reset_peek();

        match (c, lex.peek::<3>()) {
            ('m', Some(['u', 'l', '('])) => {
                if !enabled {
                    continue 'parser;
                }

                let Some((num1, next)) = lex.parse_num() else {
                    continue 'parser;
                };
                if next != ',' {
                    continue 'parser;
                }

                let Some((num2, next)) = lex.parse_num() else {
                    continue 'parser;
                };
                if next != ')' {
                    continue 'parser;
                }

                collector.push((num1, num2));
            }
            ('d', Some(['o', '(', ')'])) => {
                enabled = true;
            }
            ('d', Some(['o', 'n', '\''])) => match lex.peek::<3>() {
                Some(['t', '(', ')']) => {
                    enabled = false;
                }
                _ => continue 'parser,
            },
            (_, None) => break,
            _ => {}
        }
    }

    collector.iter().map(|(a, b)| a * b).sum::<u32>()
}

#[derive(Debug, Clone)]
struct Lexer<'a> {
    inner: &'a [char],
    pos: usize,
    peek_pos: usize,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a [char]) -> Self {
        Self {
            inner: input,
            pos: 0,
            peek_pos: 0,
        }
    }

    fn next(&mut self) -> Option<char> {
        let x = self.inner.get(self.pos).copied();
        self.pos += 1;
        x
    }

    fn peek_single(&mut self) -> Option<char> {
        if self.peek_pos == 0 {
            self.peek_pos = self.pos;
        }

        let x = self.inner.get(self.peek_pos).copied();
        self.peek_pos += 1;
        x
    }

    fn peek<const N: usize>(&mut self) -> Option<&[char; N]> {
        if self.peek_pos == 0 {
            self.peek_pos = self.pos;
        }

        if let Some(x) = self.inner.get(self.peek_pos..self.peek_pos + N) {
            let x = Some(x.try_into().unwrap());
            self.peek_pos += N;
            x
        } else {
            None
        }
    }

    fn parse_num(&mut self) -> Option<(u32, char)> {
        let mut collector = String::new();
        let last = loop {
            let Some(c) = self.peek_single() else {
                return None;
            };

            if c.is_numeric() {
                collector.push(c);
            } else {
                break c;
            }
        };

        collector.parse::<u32>().ok().map(|x| (x, last))
    }

    fn reset_peek(&mut self) {
        self.peek_pos = 0;
    }
}
