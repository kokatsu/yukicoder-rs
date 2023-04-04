fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_line(&mut s).ok();

    let res: i64 = s.trim()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .fold(0, |res, d| res + d as i64);

    println!("{}", res);
}