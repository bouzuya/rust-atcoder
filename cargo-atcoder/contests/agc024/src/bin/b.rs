use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        p: [Usize1; n],
    };

    let mut q = vec![n; n];
    for (i, p_i) in p.iter().copied().enumerate() {
        q[p_i] = i;
    }

    let mut max = 1_usize;
    let mut count = 1_usize;
    let mut prev = q[0];
    for q_i in q.into_iter().skip(1) {
        if prev < q_i {
            count += 1;
        } else {
            count = 1;
        }
        prev = q_i;
        max = max.max(count);
    }

    let ans = n - max;
    println!("{}", ans);
}
