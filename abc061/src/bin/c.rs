use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: i64,
        mut ab: [(i64, i64); n],
    };
    ab.sort();
    for &(a_i, b_i) in ab.iter() {
        k -= b_i;
        if k <= 0 {
            println!("{}", a_i);
            return;
        }
    }
}
