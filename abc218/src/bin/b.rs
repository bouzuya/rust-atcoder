use proconio::{input, marker::Usize1};

fn main() {
    input! {
        p: [Usize1; 26],
    };
    let x = "abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<char>>();
    let ans = p.into_iter().map(|p_i| x[p_i]).collect::<String>();
    println!("{}", ans);
}
