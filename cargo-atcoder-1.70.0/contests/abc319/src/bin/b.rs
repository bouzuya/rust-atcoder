use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    for i in 0..=n {
        match (1..=9).find(|j| ((n % j) == 0) && ((i % (n / j)) == 0)) {
            Some(j) => print!("{}", j),
            None => print!("-"),
        }
    }
    println!();
}
