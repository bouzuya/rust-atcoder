use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut d = vec![];
    let mut m = n;
    while m > 0 {
        d.push(m % 36);
        m /= 36;
    }
    d.reverse();
    if d.is_empty() {
        d.push(0);
    }
    let mut s = vec![];
    for &d_i in d.iter() {
        s.push(if d_i < 10 {
            '0' as u8 + d_i as u8
        } else {
            'A' as u8 + (d_i - 10) as u8
        } as char);
    }
    println!("{}", s.iter().collect::<String>());
}
