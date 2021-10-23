use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    };
    // 連結成分内に頂点と同一のもの (なしは9) がなければ -1

    let ans = n - a.len();
    println!("{}", ans);
}
