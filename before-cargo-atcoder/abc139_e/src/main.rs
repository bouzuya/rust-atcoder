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
    let mut aav = vec![vec![0usize; n - 1]; n];
    for i in 0..n {
        for j in 0..n - 2 {
            aav[i][j] = read(&mut stdin_lock, &mut buf, b' ');
        }
        aav[i][n - 2] = read(&mut stdin_lock, &mut buf, b'\n');
    }

    let mut iv = vec![(0usize, 0usize); n];
    for i in 0..n {
        iv[i].1 = i;
    }
    let mut ans = 1;
    loop {
        let mut c = false;
        let mut h = false;
        iv.sort();
        for i in 0..n {
            let (x, p) = iv[i];
            if x < n - 1 {
                c = true;
                let q = aav[p][x] - 1;
                for j in 0..n {
                    let (y, qj) = iv[j];
                    if qj == q && y < n - 1 && aav[qj][y] - 1 == p {
                        h = true;
                        iv[i].0 += 1;
                        iv[j].0 += 1;
                        break;
                    }
                }
            }
        }
        if !c {
            println!("{}", ans);
            break;
        }
        if !h {
            println!("{}", -1);
            break;
        }
        ans += 1;
    }
}
