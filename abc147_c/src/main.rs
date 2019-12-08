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
    let n: usize = read(&mut stdin_lock, &mut buf, b'\n');
    let mut av: Vec<Vec<Option<bool>>> = vec![vec![None; n]; n];
    for i in 0..n {
        let an: usize = read(&mut stdin_lock, &mut buf, b'\n');
        for _ in 0..an {
            let k: usize = read(&mut stdin_lock, &mut buf, b' ');
            let b: usize = read(&mut stdin_lock, &mut buf, b'\n');
            av[i][k - 1] = Some(b == 1usize);
        }
    }

    let mut ans = 0;
    for i in 0..2usize.pow(n as u32) {
        let mut tbl = vec![false; n];
        for j in 0..n {
            if i & (1 << j) != 0 {
                tbl[j] = true;
            }
        }

        // let mut tbl1 = vec![0i32; n];
        let mut c = false;
        for j in 0..n {
            if tbl[j] {
                for k in 0..n {
                    if let Some(b) = av[j][k] {
                        if tbl[k] != b {
                            c = true;
                        }
                    }
                }
            }
        }
        if c {
            continue;
        }

        let a = tbl.iter().filter(|&&x| x == true).count();
        ans = max(ans, a);
    }
    println!("{}", ans);
}
