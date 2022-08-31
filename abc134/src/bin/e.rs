use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut lis = vec![];
    for a_i in a.into_iter().rev() {
        let index = lis.lower_bound(&(a_i + 1));
        if index == lis.len() {
            lis.push(a_i);
        } else {
            lis[index] = a_i;
        }
    }
    let ans = lis.len();
    println!("{}", ans);
}
