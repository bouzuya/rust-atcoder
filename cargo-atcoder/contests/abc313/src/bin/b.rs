use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m]
    };
    let mut count = vec![0; n];
    for (a, b) in ab {
        count[b] += 1;
    }

    let mut zeros = vec![];
    for i in 0..n {
        if count[i] == 0 {
            zeros.push(i);
        }
    }
    if zeros.len() > 1 || zeros.is_empty() {
        println!("-1");
        return;
    }

    let ans = zeros[0] + 1;
    println!("{}", ans);
}
