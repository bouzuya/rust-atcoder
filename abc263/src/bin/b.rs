use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        p: [Usize1; n - 1],
    };
    let mut p2 = vec![0; n];
    for (i, p_i) in p.iter().copied().enumerate() {
        p2[i + 1] = p_i;
    }
    let mut count = 0;
    let mut cur = n - 1;
    while cur != 0 {
        cur = p2[cur];
        count += 1;
    }
    let ans = count;
    println!("{}", ans);
}
