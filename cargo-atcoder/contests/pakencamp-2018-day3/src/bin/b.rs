use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let s = a
        .iter()
        .scan(0, |acc, &i| {
            *acc += i;
            Some(*acc)
        })
        .collect::<Vec<usize>>();
    let ans = s.lower_bound(&(2018 + 1));
    println!("{}", ans);
}
