#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Num(usize),
    Star,
    LParen,
    Plus,
    RParen,
}

impl Token {
    fn eval(&self, a: usize, b: usize) -> usize {
        match *self {
            Token::Star => a * b,
            Token::Plus => a + b,
            _ => panic!("invalid request"),
        }
    }
}

impl From<char> for Token {
    fn from(c: char) -> Self {
        match c {
            '0'..='9' => Token::Num(c.to_digit(10).unwrap() as usize),
            '*' => Token::Star,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '+' => Token::Plus,
            _ => panic!("unknown token"),
        }
    }
}

#[aoc_generator(day18)]
pub fn generator(input: &str) -> Vec<Vec<Token>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter(|x| *x != ' ')
                .map(|x| x.into())
                .collect()
        })
        .collect()
}

fn solve1(tokens: &[Token]) -> (usize, usize) {
    let mut pos = 0;
    let mut stack = Vec::new();
    let mut op_stack: Vec<Token> = Vec::new();

    while pos < tokens.len() {
        match tokens[pos] {
            Token::Num(n) => {
                if op_stack.is_empty() {
                    stack.push(n);
                } else {
                    let m = stack.pop().unwrap();
                    let op = op_stack.pop().unwrap();
                    stack.push(op.eval(m, n))
                }
            }
            Token::Star => op_stack.push(Token::Star),
            Token::LParen => {
                let (n, new_pos) = solve1(&tokens[(pos + 1)..]);
                pos += new_pos;

                if op_stack.is_empty() {
                    stack.push(n);
                } else {
                    let m = stack.pop().unwrap();
                    let op = op_stack.pop().unwrap();
                    stack.push(op.eval(m, n))
                }
            }
            Token::Plus => op_stack.push(Token::Plus),
            Token::RParen => {
                return (stack.pop().unwrap(), pos + 1);
            }
        }
        pos += 1;
    }

    (stack.pop().unwrap(), 0)
}

fn solve2(tokens: &[Token]) -> (usize, usize) {
    let mut pos = 0;
    let mut stack = Vec::new();
    let mut op_stack: Vec<Token> = Vec::new();

    while pos < tokens.len() {
        match tokens[pos] {
            Token::Num(n) => {
                if let Some(Token::Plus) = op_stack.last() {
                    let m = stack.pop().unwrap();
                    let op = op_stack.pop().unwrap();
                    stack.push(op.eval(m, n))
                } else {
                    stack.push(n);
                }
            }
            Token::Star => op_stack.push(Token::Star),
            Token::LParen => {
                let (n, new_pos) = solve2(&tokens[(pos + 1)..]);
                pos += new_pos;

                if let Some(Token::Plus) = op_stack.last() {
                    let m = stack.pop().unwrap();
                    let op = op_stack.pop().unwrap();
                    stack.push(op.eval(m, n))
                } else {
                    stack.push(n);
                }
            }
            Token::Plus => op_stack.push(Token::Plus),
            Token::RParen => {
                while let Some(op) = op_stack.pop() {
                    let m = stack.pop().unwrap();
                    let n = stack.pop().unwrap();
                    stack.push(op.eval(m, n));
                }

                return (stack.pop().unwrap(), pos + 1);
            }
        }
        pos += 1;
    }

    while let Some(op) = op_stack.pop() {
        let m = stack.pop().unwrap();
        let n = stack.pop().unwrap();
        stack.push(op.eval(m, n));
    }

    (stack.pop().unwrap(), 0)
}

#[aoc(day18, part1)]
pub fn part1(tokens: &[Vec<Token>]) -> usize {
    tokens.iter().map(|token| solve1(token).0).sum()
}

#[aoc(day18, part2)]
pub fn part2(tokens: &[Vec<Token>]) -> usize {
    tokens.iter().map(|token| solve2(token).0).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "1 + 2 * 3 + 4 * 5 + 6";
    const SAMPLE2: &str = "1 + (2 * 3) + (4 * (5 + 6))";

    #[test]
    pub fn test_input() {
        use Token::*;
        assert_eq!(
            generator(SAMPLE2),
            vec![vec![
                Num(1),
                Plus,
                LParen,
                Num(2),
                Star,
                Num(3),
                RParen,
                Plus,
                LParen,
                Num(4),
                Star,
                LParen,
                Num(5),
                Plus,
                Num(6),
                RParen,
                RParen
            ]]
        );
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 71);
        assert_eq!(part1(&generator(SAMPLE2)), 51);
        assert_eq!(
            part1(&generator(
                "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"
            )),
            13632
        );
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 231);
        assert_eq!(part2(&generator(SAMPLE2)), 51);
        assert_eq!(part2(&generator("(2 + 4 * 9)")), 54);
        assert_eq!(part2(&generator("(6 + 9 * 8 + 6)")), 210);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2020/day18.txt");
        const ANSWERS: (usize, usize) = (45283905029161, 216975281211165);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline

            assert_eq!(part1(&generator(input)), ANSWERS.0);
            assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}
