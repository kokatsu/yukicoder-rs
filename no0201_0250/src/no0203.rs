use std::io::{stdout, Write, BufWriter};

fn main() {
    let s1: String = read_string();
    let s2: String = read_string();

    let s: String = format!("{}{}", s1, s2);

    let mut num = 0;
    let mut res = 0;
    for x in s.chars() {
        if x == 'o' {
            num += 1;
        }
        else {
            num = 0;
        }

        res = res.max(num);
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