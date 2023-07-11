use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
    };
    for a in 0..=x {
        let b = x - a;
        if (a + b == x) && (2 * a + 4 * b == y) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
