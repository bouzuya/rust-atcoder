use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n - 1],
    };
    let mut count = vec![0_usize; n];
    for a_i in a {
        count[a_i] += 1;
    }
    for c in count {
        println!("{}", c);
    }
}
