use proconio::input;

fn main() {
    input! {
        mut n: i64,
    };
    let mut s = vec![];
    while n > 0 {
        n -= 1;
        s.push(((n % 26) as u8 + b'a') as char);
        n /= 26;
    }
    println!("{}", s.iter().rev().collect::<String>());
}
