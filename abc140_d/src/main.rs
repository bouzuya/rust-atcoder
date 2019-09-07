use std::cmp::max;
use std::collections::BinaryHeap;

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
    let k: usize = read(&mut stdin_lock, &mut buf, b'\n');
    let s: String = read(&mut stdin_lock, &mut buf, b'\n');
    let cv: Vec<char> = s.chars().collect();

    let mut lc = 0i32;
    let mut rc = 0i32;
    for i in 0..n {
        if cv[i] == 'L' {
            lc += 1;
        } else {
            rc += 1;
        }
    }

    let mut lh = BinaryHeap::new();
    let mut rh = BinaryHeap::new();
    let mut c = 1;
    let mut s = 0;
    let mut e = 0;
    let mut d = cv[0];
    for i in 1..n {
        if cv[i] != d {
            if d == 'L' {
                lh.push((c, s, e));
            } else {
                rh.push((c, e, s));
            }
            c = 1;
            s = i;
            e = i;
            d = cv[i];
        } else {
            c += 1;
            e = i;
        }
    }
    if d == 'L' {
        lh.push((c, s, n - 1));
    } else {
        rh.push((c, n - 1, s));
    }

    let mut lmax = lc;
    for _ in 0..k {
        if rh.is_empty() {
            break;
        }
        lmax += rh.pop().unwrap().0;
    }
    lmax -= rh.len() as i32 + if cv[0] == 'L' { 1 } else { 0 };
    let mut h = false;
    loop {
        match rh.pop() {
            Some(x) => {
                if x.1 == n - 1 {
                    h = true
                }
            }
            None => break,
        }
    }
    lmax += if h { 1 } else { 0 };

    let mut rmax = rc;
    for _ in 0..k {
        if lh.is_empty() {
            break;
        }
        rmax += lh.pop().unwrap().0;
    }
    rmax -= lh.len() as i32 + if cv[n - 1] == 'R' { 1 } else { 0 };
    let mut h = false;
    loop {
        match lh.pop() {
            Some(x) => {
                if x.1 == 0 {
                    h = true
                }
            }
            None => break,
        }
    }
    rmax += if h { 1 } else { 0 };

    println!("{}", max(lmax, rmax));
}
