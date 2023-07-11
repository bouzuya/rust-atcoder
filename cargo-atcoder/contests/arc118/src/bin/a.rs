use proconio::input;

fn main() {
    input! {
        t: usize,
        n: usize,
    };
    let c = 100 + t;
    let f = |a| ((100 + t) * a) / 100;
    let mut x = vec![];
    let mut p = f(1);
    for a in 2..=100 {
        let f_a = f(a);
        if f_a != p + 1 {
            x.push(p + 1);
        }
        p = f_a;
    }
    let ans = (n - 1) / x.len() * c + x[(n - 1) % x.len()];
    println!("{}", ans);
}
