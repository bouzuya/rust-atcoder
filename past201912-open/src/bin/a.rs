use proconio::input;

fn main() {
    input! { s: String };
    match s.parse::<i64>() {
        Err(_) => println!("error"),
        Ok(n) => println!("{}", 2 * n),
    }
}
