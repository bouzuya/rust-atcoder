use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    };
    for x in -100..=100 {
        for y in -100..=100 {
            if x + y == a && x - y == b {
                println!("{} {}", x, y);
            }
        }
    }
}
