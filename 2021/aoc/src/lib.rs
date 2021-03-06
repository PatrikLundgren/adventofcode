use simple_error::SimpleError;
use std::io::{BufRead, BufReader};

pub fn from_lines<X, Y, Z>(input: X) -> Z
where
    X: std::io::Read,
    Y: std::str::FromStr,
    <Y as std::str::FromStr>::Err: std::fmt::Debug,
    Z: std::iter::FromIterator<Y>,
{
    BufReader::new(input)
        .lines()
        .into_iter()
        .map(|r| r.and_then(|r| Ok(r.parse::<Y>().unwrap())).unwrap())
        .collect()
}

fn bit_count(val: &str) -> (usize, usize) {
    let mut cnt = 0;
    for c in val.chars() {
        if c == '1' {
            cnt += 1;
        }
    }
    return (cnt, val.len() - cnt);
}

pub fn transpose<X>(orig: &Vec<X>, trans: &mut Vec<X>, width: usize, height: usize)
where
    X: std::marker::Copy,
{
    for c in 0..width {
        for r in 0..height {
            let elem = orig.get(c + r * width).unwrap();
            trans.push(*elem);
        }
    }
}

pub fn binary_diagnostic(diag: &[String]) -> (u64, u64) {
    let flat: Vec<char> = diag.iter().flat_map(|s| s.chars()).collect();
    let mut trans: Vec<char> = vec![];
    let mut joined: Vec<String> = vec![];
    let mut dominant: Vec<char> = vec![];

    transpose(&flat, &mut trans, diag[0].len(), diag.len());
    for (i, _) in trans.iter().step_by(diag.len()).enumerate() {
        let n = trans[i..i + diag.len()].iter().collect::<String>();
        println!("{}..{}+{}:{:?}", i, i, diag.len(), &n);
        joined.push(n);
    }

    for j in &joined {
        let (ones, zeros) = bit_count(&j);
        if ones > zeros {
            dominant.push('1')
        } else {
            dominant.push('0')
        }
    }
    println!("{:?}", trans);
    println!("{:?}", joined);
    println!("{:?}", dominant);
    return (0, 0);
}

#[derive(Clone)]
pub enum Direction {
    FORWARD,
    BACKWARD,
    DOWN,
    UP,
}

#[derive(Clone)]
pub struct Move {
    pub dir: Direction,
    pub dist: usize,
}

impl std::str::FromStr for Direction {
    type Err = SimpleError;
    fn from_str(s: &str) -> Result<Direction, Self::Err> {
        match s {
            "forward" => Ok(Direction::FORWARD),
            "backward" => Ok(Direction::BACKWARD),
            "up" => Ok(Direction::UP),
            "down" => Ok(Direction::DOWN),
            _ => Err(Self::Err::new("Failed to parse direction")),
        }
    }
}

impl std::str::FromStr for Move {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Move, Self::Err> {
        let sv: Vec<&str> = s.split(" ").collect();
        let dir = sv[0].parse::<Direction>()?;
        let dst = sv[1].parse::<usize>()?;

        Ok(Move {
            dist: dst,
            dir: dir,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
    pub aim: usize,
}

pub fn dive_aim(mut pos: Pos, moves: Vec<Move>) -> Pos {
    moves.into_iter().for_each(|m| match m.dir {
        Direction::FORWARD => {
            pos.x += m.dist;
            pos.y += pos.aim * m.dist
        }
        Direction::BACKWARD => pos.x -= m.dist,
        Direction::UP => pos.aim -= m.dist,
        Direction::DOWN => pos.aim += m.dist,
    });
    return pos;
}

pub fn dive(mut pos: Pos, moves: Vec<Move>) -> Pos {
    moves.into_iter().for_each(|m| match m.dir {
        Direction::FORWARD => pos.x += m.dist,
        Direction::BACKWARD => pos.x -= m.dist,
        Direction::UP => pos.y -= m.dist,
        Direction::DOWN => pos.y += m.dist,
    });
    return pos;
}

pub fn sonar_sweep(vals: &mut Vec<i64>, n: usize) -> usize {
    let mut last: i64 = std::i64::MAX;
    let mut cnt: usize = 0;

    for i in 0..vals.len() - (n - 1) {
        for j in 1..n {
            vals[i] += vals[i + j];
        }
    }

    for v in 0..vals.len() - (n - 1) {
        if vals[v] > last {
            cnt += 1;
        }
        last = vals[v];
    }
    return cnt;
}
