use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
        x: [usize; q],
    };
    a.sort();
    for x_i in x {
        println!("{}", a.len() - a.lower_bound(&x_i));
    }
}
