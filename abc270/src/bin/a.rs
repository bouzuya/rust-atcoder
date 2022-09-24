use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let mut ans = 0_usize;
    for i in 0..3 {
        if (a & (1 << i) != 0) || (b & (1 << i) != 0) {
            ans |= 1 << i;
        }
    }
    println!("{}", ans);
}
