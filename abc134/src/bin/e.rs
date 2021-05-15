use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    };
    a.reverse();

    let mut lis = vec![];
    for a_i in a.iter() {
        let j = lis.upper_bound(&a_i);
        if j < lis.len() {
            lis[j] = a_i;
        } else {
            lis.push(a_i);
        }
    }

    let ans = lis.len();
    println!("{}", ans);
}
