use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: [String; n],
    };
    let mut t = s[0..k].to_vec();
    t.sort();
    for s_i in t {
        println!("{}", s_i);
    }
}
