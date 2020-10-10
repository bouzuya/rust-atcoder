use proconio::input;

fn digits(n: usize) -> Vec<u8> {
    let mut d = vec![];
    let mut m = n;
    while m >= 10 {
        d.push((m % 10) as u8);
        m /= 10;
    }
    d.push((m % 10) as u8);
    d.reverse();
    d
}

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let ans = digits(a).len() * digits(b).len();
    println!("{}", ans);
}
