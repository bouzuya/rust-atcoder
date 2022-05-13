use proconio::input;

fn gcd(n: usize, m: usize) -> usize {
    if m == 0 {
        n
    } else {
        gcd(m, n % m)
    }
}

fn main() {
    input! {
        n: usize,
        capital_x: usize,
        mut x: [usize; n],
    };
    x.push(capital_x);
    x.sort();
    let mut d = x[1] - x[0];
    let mut p = x[1];
    for x_i in x.into_iter().skip(2) {
        let d_i = x_i - p;
        d = gcd(d, d_i);
        p = x_i;
    }
    let ans = d;
    println!("{}", ans);
}
