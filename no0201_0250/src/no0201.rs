use std::io::{stdout, Write, BufWriter};

fn main() {
    let a: Vec<String> = read_vec();
    let b: Vec<String> = read_vec();

    let res: &str = {
        if a[1].len() == b[1].len() {
            if a[1] > b[1] {
                &a[0]
            }
            else if a[1] < b[1] {
                &b[0]
            }
            else {
                "-1"
            }
        }
        else if a[1].len() > b[1].len() {
            &a[0]
        }
        else {
            &b[0]
        }
    };

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