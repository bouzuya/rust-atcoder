use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        q: usize,
        b: [usize; q],
    };

    a.sort();

    for b_i in b {
        let j = a.lower_bound(&b_i);
        let mut min = std::i64::MAX;
        if let Some(&b_j) = a.get(j.saturating_sub(1)) {
            min = min.min((b_i as i64 - b_j as i64).abs());
        }
        if let Some(&b_j) = a.get(j) {
            min = min.min((b_i as i64 - b_j as i64).abs());
        }
        if let Some(&b_j) = a.get(j + 1) {
            min = min.min((b_i as i64 - b_j as i64).abs());
        }
        println!("{}", min);
    }
}
