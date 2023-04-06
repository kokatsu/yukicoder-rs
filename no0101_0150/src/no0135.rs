fn main() {
    let _n: usize = read();
    let mut x: Vec<i32> = read_vec();

    x.sort();
    x.dedup();

    let l: usize = x.len();

    let mut res: i32 = if l == 1 {0} else {x[l-1]};
    for i in 0..l-1 {
        res = res.min(x[i+1]-x[i]);
    }

    println!("{}", res)
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