use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    };
    let mut count = vec![0_usize; 3];
    for a_i in a {
        count[a_i] += 1;
    }

    let mut sum = 0_usize;
    for c in count {
        if c == 0 {
            continue;
        }
        sum += c * (c - 1) / 2;
    }

    let ans = sum;
    println!("{}", ans);
}
