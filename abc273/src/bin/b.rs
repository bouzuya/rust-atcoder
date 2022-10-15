use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut x: Chars,
        k: usize,
    };
    x.reverse();
    let mut c = 0_usize;
    let mut ans = vec![];
    for (i, x_i) in x.iter().copied().enumerate() {
        let d = (x_i as u8 - b'0') as usize + c;
        if i < k {
            ans.push('0');
            if d <= 4 {
                c = 0;
            } else {
                c = 1;
            }
        } else {
            ans.push(((d % 10) as u8 + b'0') as char);
            c = d / 10;
        }
    }
    if c > 0 && k <= x.len() {
        ans.push('1');
    }
    ans.reverse();
    println!(
        "{}",
        ans.iter().collect::<String>().parse::<usize>().unwrap()
    );
}
