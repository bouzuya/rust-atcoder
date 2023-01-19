use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [usize; n],
    };
    let mut y = x.clone();
    y.sort();
    let l = y[n / 2 - 1];
    let r = y[n / 2];
    for x_i in x {
        if x_i <= l {
            println!("{}", r);
        } else {
            println!("{}", l);
        }
    }
}
