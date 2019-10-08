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
    let mut sv: Vec<String> = Vec::new();
    for _ in 0..n {
        sv.push(read(&mut stdin_lock, &mut buf, b'\n'));
    }

    let mut cv = vec![0i32; 5];
    for i in 0..n {
        let s: Vec<char> = sv[i].chars().collect();
        match s[0] {
            'M' => cv[0] += 1,
            'A' => cv[1] += 1,
            'R' => cv[2] += 1,
            'C' => cv[3] += 1,
            'H' => cv[4] += 1,
            _ => {}
        }
    }

    let mut ans = 0i64;
    for i in 0..2usize.pow(5) {
        let mut c = 0;
        let mut x = 1i64;
        for j in 0..cv.len() {
            if i & (1 << j) != 0 {
                x *= cv[j] as i64;
                c += 1;
            }
        }
        if c == 3 {
            ans += x;
        }
    }
    println!("{}", ans);
}
