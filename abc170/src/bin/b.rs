use proconio::input;

fn main() {
    input! {
        x: i64,
        y: i64,
    };
    for a in 0..=100 {
        for b in 0..=100 {
            if (a + b == x) && (a * 2 + b * 4 == y) {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
