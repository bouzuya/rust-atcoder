use proconio::input;

fn main() {
    input! {
        d: usize,
    };
    print!("Christmas");
    for _ in d..=24 {
        print!(" Eve");
    }
    println!();
}
