use proconio::input;

fn main() {
    input! {
        n: usize,
        _: usize,
        mut s: [String; n],
    };
    s.sort();
    let mut ans = String::new();
    for s_i in s.iter() {
        ans += s_i;
    }
    println!("{}", ans);
}
