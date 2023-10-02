use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        m: usize,
        p: i64,
        a: [i64; n],
        mut b: [i64; m],
    };

    b.sort();

    let s = std::iter::once(0)
        .chain(b.iter().scan(0, |acc, &i| {
            *acc += i;
            Some(*acc)
        }))
        .collect::<Vec<i64>>();

    let mut sum = 0_i64;
    for a_i in a {
        if a_i >= p {
            sum += p * m as i64;
            continue;
        }

        let count = b.lower_bound(&(p - a_i));
        sum += s[count] + a_i * count as i64;
        sum += (m - count) as i64 * p;
    }
    let ans = sum;
    println!("{}", ans);
}
