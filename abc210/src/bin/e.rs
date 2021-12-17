use proconio::input;

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        mut ac: [(usize, usize); m],
    };
    ac.sort_by_key(|&(_, c)| c);
    let mut x_i = n;
    let mut x_p = n;
    let mut ans = 0_usize;
    for (a_i, c_i) in ac {
        x_i = gcd(x_i, a_i);
        ans += c_i * (x_p - x_i);
        x_p = x_i;
    }
    if x_i > 1 {
        println!("{}", -1);
        return;
    }
    println!("{}", ans);
}
