use proconio::input;

fn main() {
    input! {
        a: [usize; 64],
    };
    let mut ans = 0_usize;
    for i in 0..64 {
        ans |= a[i] << i;
    }
    println!("{}", ans);
}
