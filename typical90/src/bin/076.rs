use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    };
    let sum = a.iter().sum::<u64>();
    let s = std::iter::once(0)
        .chain(a.iter().chain(a.iter()).scan(0, |acc, &i| {
            *acc += i;
            Some(*acc)
        }))
        .collect::<Vec<u64>>();
    for (i, &s_i) in s.iter().enumerate() {
        let s_j = sum / 10 + s_i;
        let j = s.lower_bound(&s_j);
        if j < s.len() && (s[j] - s[i]) * 10 == sum {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
