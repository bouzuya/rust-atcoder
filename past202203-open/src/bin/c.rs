use proconio::{input, marker::Chars};

fn main() {
    input! {
        alpha: Chars,
    };
    let n = alpha.len();
    let x = (n - 1) / 3 - 1;
    let y = n - (n - 1) / 3 * 3;
    let mut ans = vec![];
    for i in 0..y {
        ans.push(alpha[i]);
    }
    ans.push((x as u8 + b'a') as char);
    let ans = ans.iter().collect::<String>();
    println!("{}", ans);
}
