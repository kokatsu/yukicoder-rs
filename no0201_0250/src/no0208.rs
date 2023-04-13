use std::io::{stdout, Write, BufWriter};

fn main() {
    let input1: Vec<i64> = read_vec();
    let x: i64 = input1[0];
    let y: i64 = input1[1];

    let input2: Vec<i64> = read_vec();
    let x2: i64 = input2[0];
    let y2: i64 = input2[1];

    let mut res: i64 = x.max(y);
    if x > x2 && x == y && x2 == y2 {
        res += 1;
    }

    let mut out = BufWriter::new(stdout().lock());
    writeln!(out, "{}", res).unwrap();
}

#[allow(dead_code)]
fn read_string() -> String {
    let mut s: String = String::new();
    std::io::stdin().read_line(&mut s).ok();

    s.trim().to_string()
}

#[allow(dead_code)]
fn read<T: std::str::FromStr>() -> T {
    read_string().parse().ok().unwrap()
}

#[allow(dead_code)]
fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read_string()
        .split_whitespace()
        .map(|v| v.parse().ok().unwrap())
        .collect()
}