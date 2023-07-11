use proconio::input;

fn main() {
    input! {
        a: usize,
    };
    // 1
    // 1 1
    // 1 2 ...
    // 1 3 ...
    // 1 4 ...
    // ...
    println!("{} {}", a + 1, 2);
}
