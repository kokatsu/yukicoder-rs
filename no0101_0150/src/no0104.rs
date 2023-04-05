fn main() {
    let s: String = read_string();

    let res: i64 = s.chars()
                    .map(|d| d)
                    .fold(1, |mut res, d| {
                        res *= 2;
                        if d == 'R' {
                            res += 1;
                        }
                        res
                    });

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