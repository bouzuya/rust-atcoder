use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut r: [usize; n],
        query: [usize; q],
    };
    r.sort();

    for i in 1..r.len() {
        r[i] += r[i - 1];
    }

    for x in query {
        println!("{}", r.lower_bound(&(x + 1)));
    }
}
