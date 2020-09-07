use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: usize,
        b: usize,
    };
    for r in 0..h {
        for c in 0..w {
            print!(
                "{}",
                match (r < b, c < a) {
                    (true, true) => 0,
                    (true, false) => 1,
                    (false, true) => 1,
                    (false, false) => 0,
                }
            );
        }
        println!();
    }
}
