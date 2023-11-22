use proconio::{input, marker::Chars};

fn main() {
    input! {
        d: usize,
        mut a: Chars,
        mut b: Chars,
    };

    let len = a.len().max(b.len());
    a.reverse();
    b.reverse();
    let mut c = 0;
    let mut ans = vec![];
    for i in 0..len {
        if i == d {
            ans.push('.');
            continue;
        }
        let x = (*a.get(i).unwrap_or(&'0') as u8 - b'0') as usize;
        let y = (*b.get(i).unwrap_or(&'0') as u8 - b'0') as usize;
        ans.push((((x + y + c) % 10) as u8 + b'0') as char);
        c = (x + y + c) / 10;
    }
    if c != 0 {
        ans.push((c as u8 + b'0') as char);
    }

    ans.reverse();

    println!("{}", ans.into_iter().collect::<String>());
}
