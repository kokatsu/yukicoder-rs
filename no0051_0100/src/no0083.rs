fn main() {
    let n: usize = read();

    let r: usize = n % 2;
    let d: usize = if r == 1 {n / 2 - 1} else {n / 2};

    let mut res: String = if r == 1 {String::from("7")} else {String::new()};
    res += &String::from("1").repeat(d);

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