use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    println!("{}", n);
    for _ in 0..n {
        println!("1");
    }
}
