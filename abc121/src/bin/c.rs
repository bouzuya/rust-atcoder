use proconio::input;

fn main() {
    input! {
        n: usize,
        m: u64,
        mut ab: [(u64, u64); n]
    };
    ab.sort();
    let mut c = 0;
    let mut ans = 0;
    for &(a_i, b_i) in ab.iter() {
        if c + b_i >= m {
            ans += a_i * (m - c);
            break;
        } else {
            c += b_i;
            ans += a_i * b_i;
        }
    }
    println!("{}", ans);
}
