fn main() {
    let l: i32 = read_string().parse().unwrap();
    let _n: i32 = read_string().parse().unwrap();
    let mut w: Vec<i32> = read_string().split_whitespace()
                                        .map(|v| v.parse().unwrap())
                                        .collect();

    w.sort();

    let mut num: i32 = 0;
    let mut res: i32 = 0;
    for v in w {
        num += v;
        if num <= l {
            res += 1;
        }
    }

    println!("{}", res);
}

fn read_string() -> String {
    let mut s: String = String::new();
    std::io::stdin().read_line(&mut s).ok();

    s.trim().to_string()
}