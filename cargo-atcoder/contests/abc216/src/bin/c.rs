use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut s = vec![];
    let mut m = n;
    while m > 0 {
        if m % 2 == 0 {
            s.push('B');
            m /= 2;
        } else {
            s.push('A');
            m -= 1;
        }
    }
    s.reverse();
    println!("{}", s.iter().collect::<String>());
}
