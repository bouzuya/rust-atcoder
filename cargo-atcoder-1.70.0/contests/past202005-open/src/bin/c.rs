use proconio::input;

fn main() {
    input! {
        a: usize,
        r: usize,
        n: usize,
    };

    if r == 1 {
        let x = a * r;
        if x > 10_usize.pow(9) {
            println!("large");
        } else {
            println!("{}", x);
        }
        return;
    }

    let mut x = a;
    for _ in 1..n {
        x *= r;
        if x > 10_usize.pow(9) {
            println!("large");
            return;
        }
    }
    let ans = x;
    println!("{}", ans);
}
