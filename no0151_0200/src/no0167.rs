fn main() {
    let n: String = read_string();
    let m: String = read_string();

    if m == String::from("0") {
        println!("{}", 1);
        return;
    }

    let u: Vec<char> = n.chars().collect();
    let v: Vec<char> = m.chars().collect();

    let b: u32 = u.last().unwrap().to_digit(10).unwrap();

    let l: usize = m.len();
    let mut p: u32 = v[l-1].to_digit(10).unwrap() + 4;
    if l > 1 {
        p += v[l-2].to_digit(10).unwrap() * 10;
    }

    let mut res: u32 = 1;
    for _ in 1..=p {
        res = (res * b) % 10;
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