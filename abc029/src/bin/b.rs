use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: [Chars; 12],
    };
    let ans = s.iter().filter(|s_i| s_i.contains(&'r')).count();
    println!("{}", ans);
}
