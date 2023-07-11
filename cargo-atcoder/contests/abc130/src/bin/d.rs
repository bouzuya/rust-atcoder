use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    };
    let s = std::iter::once(0)
        .chain(a.iter().scan(0, |acc, &i| {
            *acc += i;
            Some(*acc)
        }))
        .collect::<Vec<usize>>();
    let mut sum = 0_usize;
    for s_i in s.iter().copied().take(n) {
        sum += n + 1 - s.lower_bound(&(s_i + k));
    }
    let ans = sum;
    println!("{}", ans);
}
