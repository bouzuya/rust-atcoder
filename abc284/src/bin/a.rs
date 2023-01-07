use proconio::input;

fn main() {
    input! {
        n: usize,
        mut s: [String; n],
    };
    s.reverse();
    for s_i in s {
        println!("{}", s_i);
    }
}
