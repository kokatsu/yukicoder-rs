use std::io::{stdout, Write, BufWriter};

fn check(x: Vec<i64>) -> bool {
    if x[0] > x[1] && x[2] > x[1] && x[0] != x[2] {
        return true;
    }
    if x[0] < x[1] && x[2] < x[1] && x[0] != x[2] {
        return true;
    }
    false
}

fn main() {
    let mut a: Vec<i64> = read_vec();
    let mut b: Vec<i64> = read_vec();

    let mut is_ok: bool = false;
    for i in 0..3 {
        for j in 0..3 {
            std::mem::swap(&mut a[i], &mut b[j]);
            is_ok |= check(a.clone()) && check(b.clone());
            std::mem::swap(&mut a[i], &mut b[j]);
        }
    }

    let res: &str = if is_ok {"Yes"} else {"No"};

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