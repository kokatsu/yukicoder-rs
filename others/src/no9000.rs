fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_line(&mut s).ok();

    let res: &str = "Hello World!";
    println!("{}", res);
}