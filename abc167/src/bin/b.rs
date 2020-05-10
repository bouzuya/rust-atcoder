use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        k: i64,
    };

    let mut rk = k;
    let na = std::cmp::min(rk, a);
    rk -= na;
    let nb = std::cmp::min(rk, b);
    rk -= nb;
    let nc = std::cmp::min(rk, c);
    let ans = na - nc;
    println!("{}", ans);
}
