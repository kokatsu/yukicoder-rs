fn main() {
    let f: Vec<i64> = read_vec();

    let res: i64 = match f {
        f if f[2] % 3 == 0 => f[0],
        f if f[2] % 3 == 1 => f[1],
        _ => f[0] ^ f[1],
    };

    println!("{}", res);
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