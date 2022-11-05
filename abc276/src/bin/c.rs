use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        mut p: [usize; n],
    };

    p.prev_permutation();
    for (i, p_i) in p.iter().copied().enumerate() {
        print!("{}{}", p_i, if i == n - 1 { '\n' } else { ' ' });
    }
}
