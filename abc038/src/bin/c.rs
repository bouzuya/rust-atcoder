use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut ans = 1_u64;
    let mut c = 0_u64;
    let mut a_p = a[0];
    for &a_i in a.iter().skip(1) {
        if a_p < a_i {
            c += 1;
        } else {
            c = 0;
        }
        ans += c + 1;
        a_p = a_i;
    }
    println!("{}", ans);
}
