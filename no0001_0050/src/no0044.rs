fn main() {
    let n: usize = read();

    let mut dp: Vec<i64> = vec![0; n+1];
    dp[0] = 1;
    for i in 1..=n {
        dp[i] += dp[i-1];
        if i > 1 {
            dp[i] += dp[i-2];
        }
    }

    let res: i64 = dp[n];
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