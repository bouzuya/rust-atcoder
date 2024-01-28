use proconio::{input, marker::Usize1};

fn main() {
    input! {
        p: [Usize1; 26],
    };
    let cs = "abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<char>>();
    let ans = p.into_iter().map(|p_i| cs[p_i]).collect::<String>();
    println!("{}", ans);
}
