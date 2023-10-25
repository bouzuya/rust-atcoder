use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    };
    let mut min = s.clone();
    let mut max = s.clone();
    let n = s.len();
    for _ in 0..n {
        if s < min {
            min = s.clone();
        }
        if s > max {
            max = s.clone();
        }
        s.rotate_left(1);
    }
    println!("{}", min.into_iter().collect::<String>());
    println!("{}", max.into_iter().collect::<String>());
}
