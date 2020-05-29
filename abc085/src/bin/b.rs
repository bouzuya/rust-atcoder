use proconio::input;

fn main() {
    input! {
        n: usize,
        mut d: [u64; n],
    };
    d.sort();
    let mut x = 0;
    let mut ans = 0;
    for &d_i in d.iter() {
        if d_i > x {
            x = d_i;
            ans += 1;
        }
    }
    println!("{}", ans);
}
