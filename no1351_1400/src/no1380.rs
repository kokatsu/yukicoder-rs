use std::io::{stdout, Write, BufWriter};

fn main() {
    let input: Vec<i64> = read_vec();
    let k: i64 = input[1];

    let mut p: Vec<i64> = read_vec();

    p.sort_by_key(|&x| std::cmp::Reverse(x));

    let g: Vec<(i64, i64)> = run_length_encoding(p);

    let mut res: i64 = 0;
    for (_, x) in g {
        if res + x > k {
            break;
        }
        res += x;
    }

    let mut out = BufWriter::new(stdout().lock());
    writeln!(out, "{}", res).unwrap();
}

#[allow(dead_code)]
fn run_length_encoding<T: std::cmp::PartialEq + Copy>(x: Vec<T>) -> Vec<(T, i64)> {
    let mut ret: Vec<(T, i64)> = Vec::new();

    let l: usize = x.len();

    let mut i: usize = 0;
    let mut j: usize = 0;
    while i < l {
        while j < l && x[i] == x[j] {
            j += 1usize;
        }

        ret.push((x[i], (j-i) as i64));
        i = j;
    }

    ret
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