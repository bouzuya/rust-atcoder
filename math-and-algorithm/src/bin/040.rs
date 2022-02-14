use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [usize; n - 1],
        m: usize,
        b: [Usize1; m],
    };

    let c = std::iter::once(0)
        .chain(a.iter().scan(0, |acc, &i| {
            *acc += i;
            Some(*acc)
        }))
        .collect::<Vec<usize>>();

    let mut ans = 0_usize;
    let mut prev = b[0];
    for b_i in b.into_iter().skip(1) {
        let (l, r) = if b_i < prev { (b_i, prev) } else { (prev, b_i) };
        ans += c[r] - c[l];
        prev = b_i;
    }

    println!("{}", ans);
}
