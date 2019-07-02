use std::cmp::max;

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
    let n: usize = read(&mut stdin_lock, &mut buf, b' ');
    let w: usize = read(&mut stdin_lock, &mut buf, b'\n');
    let mut ans = vec![vec![0i64; w + 1]; n + 1];
    for i in 1..n + 1 {
        let wi: usize = read(&mut stdin_lock, &mut buf, b' ');
        let vi: i64 = read(&mut stdin_lock, &mut buf, b'\n');
        for j in 1..w + 1 {
            ans[i][j] = max(
                max(ans[i - 1][j], ans[i][j - 1]),
                if j < wi { 0 } else { ans[i - 1][j - wi] + vi },
            );
        }
    }
    println!("{}", ans[n][w]);
}
