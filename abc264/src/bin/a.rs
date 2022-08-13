use proconio::{input, marker::Usize1};

fn main() {
    input! {
        l: Usize1,
        r: Usize1,
    };
    let ans = "atcoder".chars().collect::<Vec<char>>()[l..r + 1]
        .iter()
        .collect::<String>();
    println!("{}", ans);
}
