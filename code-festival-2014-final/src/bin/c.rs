use proconio::input;

fn f(n: i64) -> i64 {
    let mut d = vec![];
    let mut m = n;
    while m > 0 {
        d.push(m % 10);
        m /= 10;
    }
    d.reverse();
    let mut res = 0_i64;
    for d_i in d.iter() {
        res *= n;
        res += d_i;
    }
    res
}

fn main() {
    input! {
        a: i64,
    };
    for n in 10..=10000 {
        if f(n) == a {
            println!("{}", n);
            return;
        }
    }
    println!("-1");
}
