fn main() {
    let _n: i64 = read();
    let mut l: Vec<i64> = read_vec();

    l.sort();

    let mut g: Vec<(i64, i64)> = run_length_encoding(l);

    g.sort_by(|a, b| {
        if a.1 == b.1 {
            return (-a.0).partial_cmp(&(-b.0)).unwrap();
        }
        (-a.1).partial_cmp(&(-b.1)).unwrap()
    });

    let res: i64 = g[0].0;

    println!("{}", res);
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