use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut s: [String; n],
    };
    s.sort();
    let mut t = String::new();
    for s_i in s.iter().take(k) {
        t.push_str(s_i);
    }
    let ans = t;
    println!("{}", ans);
}
