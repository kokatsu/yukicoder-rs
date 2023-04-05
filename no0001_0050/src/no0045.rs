fn main() {
    let n: usize = read();
    let v: Vec<i32> = read_vec();

    let mut dp: Vec<i32> = vec![0; n+1];
    for i in 0..n {
        if i == 0 {
            dp[i+1] = v[i];
        }
        else {
            dp[i+1] = dp[i].max(dp[i-1]+v[i]);
        }
    }

    let res: i32 = dp[n];
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