use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        av: [Usize1; n - 1],
    };
    let mut cv = vec![0_usize; n];
    for a in av {
        cv[a] += 1;
    }
    for c in cv {
        println!("{}", c);
    }
}
