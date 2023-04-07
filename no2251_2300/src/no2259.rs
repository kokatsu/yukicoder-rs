fn main() {
    let v: Vec<i64> = read_vec();

    let l: i64 = v[0];
    let r: i64 = v[1];
    let c: i64 = v[2];

    let d: i64 = 1_000;

    let mut num: i64 = c % d;
    let mut count: i64 = 0;
    let mut mn: i64 = d;
    let mut seen: Vec<bool> = vec![false; d as usize];
    while !seen[num as usize] {
        seen[num as usize] = true;
        mn = mn.min((d-num)%d);
        num = (num + c) % d;
        count += 1;
    }

    if r - l >= count {
        println!("{}", mn);
        return;
    }

    let mut res: i64 = d;
    num = (l * c) % d;
    for _ in 0..=r-l {
        res = res.min((d-num)%d);
        num = (num + c) % d;
    }

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