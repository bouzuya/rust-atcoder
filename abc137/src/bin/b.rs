use proconio::input;

fn main() {
    input! {
        k: i64,
        x: i64,
    };
    for i in (x - k + 1)..=(x + k - 1) {
        println!("{}", i);
    }
}
