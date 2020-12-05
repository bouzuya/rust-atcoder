use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [i64; n],
        mut b: [i64; m],
    };
    a.sort();
    a.reverse();
    b.sort();
    b.reverse();
    let mut ans = m <= n;
    for (&b_i, &a_i) in b.iter().zip(a.iter()) {
        if b_i > a_i {
            ans = false;
            break;
        }
    }
    println!("{}", if ans { "YES" } else { "NO" });
}
