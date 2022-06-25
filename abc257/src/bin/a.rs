use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        x: Usize1,
    };
    let mut s = String::new();
    for i in 0..26 {
        s.push_str(&(('A' as u8 + i as u8) as char).to_string().repeat(n));
    }
    let ans = s.chars().collect::<Vec<char>>()[x];
    println!("{}", ans);
}
