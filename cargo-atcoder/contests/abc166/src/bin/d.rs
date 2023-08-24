use proconio::input;

fn main() {
    input! {
        x: i64,
    };
    for a in -1000_i64..=1000 {
        for b in -1000_i64..=1000 {
            if a.pow(5) - b.pow(5) == x {
                println!("{} {}", a, b);
                return;
            }
        }
    }
}
