use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
        a: [usize; n],
    };
    for a_i in a {
        println!("{}", a_i.clamp(l, r));
    }
}
