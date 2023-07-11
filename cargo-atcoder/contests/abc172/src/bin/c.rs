use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        a: [usize; n],
        b: [usize; m],
    };

    let a = std::iter::once(0)
        .chain(a.iter().scan(0, |acc, &i| {
            *acc += i;
            Some(*acc)
        }))
        .collect::<Vec<usize>>();
    let b = std::iter::once(0)
        .chain(b.iter().scan(0, |acc, &i| {
            *acc += i;
            Some(*acc)
        }))
        .collect::<Vec<usize>>();

    let mut max = 0_usize;
    for (i, a_i) in a.iter().copied().enumerate() {
        if a_i > k {
            continue;
        }
        let j = b.lower_bound(&(k - a_i + 1)) - 1;
        max = max.max(i + j);
    }
    let ans = max;
    println!("{}", ans);
}
