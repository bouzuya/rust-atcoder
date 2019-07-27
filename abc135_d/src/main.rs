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
    let s: String = read(&mut stdin_lock, &mut buf, b'\n');
    let cv: Vec<char> = s.chars().collect();
    let l = cv.len();

    let mut dp = vec![vec![0u32; 13]; l + 1];
    dp[0][0] = 1;
    for i in 0..l {
        let c = cv[i];
        if c == '?' {
            for d in 0..10 {
                for j in 0..13 {
                    dp[i + 1][(j * 10 + d) % 13] += dp[i][j];
                    dp[i + 1][(j * 10 + d) % 13] %= 1_000_000_007;
                }
            }
        } else {
            let d = c.to_digit(10).unwrap() as usize;
            for j in 0..13 {
                dp[i + 1][(j * 10 + d) % 13] += dp[i][j];
                dp[i + 1][(j * 10 + d) % 13] %= 1_000_000_007;
            }
        }
    }
    let ans = dp[l][5];
    println!("{}", ans);
}
