fn main() {
    let mut input1: String = String::new();
    std::io::stdin().read_line(&mut input1).ok();

    let array: Vec<_> = input1.trim().split(' ').collect();
    let a: i64 = array[0].parse().unwrap();
    let b: i64 = array[1].parse().unwrap();

    let n: i64 = a + b;

    let mut input2: String = String::new();
    std::io::stdin().read_line(&mut input2).ok();

    let s: &str = input2.trim();

    println!("{} {}", n, s);
}