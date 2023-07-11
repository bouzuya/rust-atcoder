use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: usize,
        b: usize,
    };
    for i in 0..h {
        for j in 0..w {
            print!(
                "{}",
                match (i < b, j < a) {
                    (true, true) | (false, false) => 0,
                    (true, false) | (false, true) => 1,
                }
            );
        }
        println!();
    }
}
