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

    let mut ct = vec![vec![0; 10]; 10];
    for i in 1..n + 1 {
        let t = i % 10;
        let mut h = i;
        while h >= 10 {
            h /= 10;
        }
        ct[h][t] += 1;
    }

    let mut ans: u64 = 0;
    for i in 0..10 {
        for j in 0..10 {
            ans += ct[i][j] * ct[j][i];
        }
    }
    println!("{}", ans);
}
