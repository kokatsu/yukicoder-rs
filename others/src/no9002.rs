fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_line(&mut s).ok();

    let n: i64 = s.trim().parse().unwrap();

    for i in 1..=n {
        let t: String;
        let s: &str = match i {
            i if i % 15 == 0 => "FizzBuzz",
            i if i % 3 == 0 => "Fizz",
            i if i % 5 == 0 => "Buzz",
            i => {t = i.to_string(); &t},
        };
        println!("{}", s);
    }
}