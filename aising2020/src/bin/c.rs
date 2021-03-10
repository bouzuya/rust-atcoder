use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
    };
    let mut vs = vec![];
    for x in 1..=100 {
        for y in 1..=100 {
            for z in 1..=100 {
                vs.push(x * x + y * y + z * z + x * y + y * z + z * x);
            }
        }
    }
    vs.sort();
    for x in 1..=n {
        println!("{}", vs.upper_bound(&x) - vs.lower_bound(&x));
    }
}
