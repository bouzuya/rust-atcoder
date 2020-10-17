use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [i64; n],
    };
    let mut m = 0;
    for &x_i in x.iter() {
        m += x_i.abs();
    }
    let mut u = 0;
    for &x_i in x.iter() {
        u += x_i.pow(2);
    }
    let u = (u as f64).sqrt();
    let mut c = x[0].abs();
    for &x_i in x.iter() {
        c = std::cmp::max(c, x_i.abs());
    }
    println!("{}", m);
    println!("{}", u);
    println!("{}", c);
}
