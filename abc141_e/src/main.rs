use std::cmp::{max, min};

fn read<T: std::str::FromStr>(
    stdin_lock: &mut std::io::StdinLock,
    buf: &mut Vec<u8>,
    delimiter: u8,
) -> T {
    buf.clear();
    let l = std::io::BufRead::read_until(stdin_lock, delimiter, buf).unwrap();
    buf.truncate(l - 1); // remove delimiter
    let s = unsafe { std::str::from_utf8_unchecked(&buf) };
    s.parse().unwrap_or_else(|_| panic!("read"))
}

fn main() {
    let stdin = std::io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut buf: Vec<u8> = Vec::new();
    let n: usize = read(&mut stdin_lock, &mut buf, b'\n');
    let s: String = read(&mut stdin_lock, &mut buf, b'\n');
    let cv: Vec<char> = s.chars().collect();

    let mut tv = vec![-1i32; 26];
    let mut iv = vec![-1i32; cv.len()];
    for i in 0..n {
        let j = n - i - 1;
        let index = (cv[j] as usize) - ('a' as usize);
        iv[j] = tv[index];
        tv[index] = j as i32;
    }

    let mut ans = 0;
    let mut c = 0;
    for i in 0..n - 1 {
        if iv[i] + 1 == iv[i + 1] {
            c = if c == 0 { 2 } else { c + 1 };
        } else {
            c = 0;
        }
        ans = max(ans, c);
    }
    println!("{}", min(ans, n / 2));
}
