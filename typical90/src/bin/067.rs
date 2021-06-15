use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut n: Chars,
        k: usize,
    };
    for _ in 0..k {
        let mut d = u64::from_str_radix(n.iter().collect::<String>().as_str(), 8).unwrap();
        if d == 0 {
            println!("0");
            return;
        }
        let mut s = vec![];
        while d > 0 {
            let d_i = (d % 9) as u8;
            let d_i = if d_i == 8 { 5 } else { d_i };
            s.push((d_i + b'0') as char);
            d /= 9;
        }
        s.reverse();
        n = s;
    }
    let ans = n.iter().collect::<String>();
    println!("{}", ans);
}
