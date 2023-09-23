use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        m: usize,
        p: usize,
        mut a: [usize; n],
        mut b: [usize; m],
    };
    a.sort();
    b.sort();

    let c = std::iter::once(0)
        .chain(b.iter().scan(0, |acc, &i| {
            *acc += i;
            Some(*acc)
        }))
        .collect::<Vec<usize>>();

    let mut sum = 0_usize;
    for a_i in a.iter().copied() {
        if a_i >= p {
            sum += p * m;
            continue;
        }
        let x = p - a_i;
        let count = b.lower_bound(&(x + 1));
        sum += c[count] + (count * a_i) + (m - count) * p;
    }
    let ans = sum;
    println!("{}", ans);
}
