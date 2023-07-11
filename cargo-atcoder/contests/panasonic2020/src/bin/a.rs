use proconio::{input, marker::Usize1};

fn main() {
    input! {
        k: Usize1,
    };
    let a = vec![
        1, 1, 1, 2, 1, 2, 1, 5, 2, 2, 1, 5, 1, 2, 1, 14, 1, 5, 1, 5, 2, 2, 1, 15, 2, 2, 5, 4, 1, 4,
        1, 51,
    ];
    let ans = a[k];
    println!("{}", ans);
}
