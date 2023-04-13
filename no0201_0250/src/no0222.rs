use std::io::{stdout, Write, BufWriter};

fn main() {
    let s: String = read_string();

    let mut res: i64 = 0;
    let mut sign: i64 = 1;
    let mut num: i64 = 0;
    let mut op: i64 = 0;
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (c == '+' || c == '-') && op == 0 {
            res += sign * num;
            sign = 1;
            num = 0;

            if c == '+' {
                op = -1;
            }
            else {
                op = 1;
            }
        }
        else if c == '+' {
            sign = 1;
        }
        else if c == '-' {
            sign = -1;
        }
        else {
            num = num * 10 + (c as i64) - ('0' as i64);
        }
    }

    res += op * sign * num;

    let mut out = BufWriter::new(stdout().lock());
    writeln!(out, "{:?}", res).unwrap();
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