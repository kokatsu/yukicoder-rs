use std::io::{stdout, Write, BufWriter};

fn main() {
    let input: Vec<i64> = read_vec();
    let a: i64 = input[0];
    let b: i64 = input[1];

    let mut out = BufWriter::new(stdout().lock());

    for i in a..=b {
        let mut n = i;

        let mut is_ok = n % 3 == 0;
        while n > 0 {
            if n % 10 == 3 {
                is_ok = true;
            }
            n /= 10;
        }

        if is_ok {
            writeln!(out, "{}", i).unwrap();
        }
    }
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