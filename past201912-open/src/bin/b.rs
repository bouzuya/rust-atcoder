use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    for w in a.windows(2) {
        match w {
            &[a_i, a_j] => {
                if a_j == a_i {
                    println!("stay");
                } else if a_j < a_i {
                    println!("down {}", a_i - a_j);
                } else if a_j > a_i {
                    println!("up {}", a_j - a_i);
                } else {
                    unreachable!();
                }
            }
            _ => unreachable!(),
        }
    }
}
