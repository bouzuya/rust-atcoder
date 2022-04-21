use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        x: Usize1,
        y: Usize1,
    };
    let mut count = vec![0; n];
    for i in 0..n {
        for j in i + 1..n {
            let d = (j - i)
                .min(if i < x { x - i } else { i - x } + 1 + if j < y { y - j } else { j - y });
            count[d] += 1;
        }
    }
    for i in 1..=n - 1 {
        println!("{}", count[i]);
    }
}
