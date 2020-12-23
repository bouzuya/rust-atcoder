use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        mut l: [i64; n],
    };
    l.sort();
    let mut sum = 0;
    for i in 0..n {
        let a = l[i];
        for j in i + 1..n {
            let b = l[j];
            let k = l.lower_bound(&(a + b));
            sum += k - (j + 1);
        }
    }
    let ans = sum;
    println!("{}", ans);
}
