fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_line(&mut s).ok();

    let n: i64 = s.trim().parse().unwrap();

    let s: &str = match n {
        n if n % 400 == 0 => "Yes",
        n if n % 100 == 0 => "No",
        n if n % 4 == 0 => "Yes",
        _ => "No",
    };

    println!("{}", s);
}