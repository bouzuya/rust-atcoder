use std::cmp::max;
use std::collections::VecDeque;

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
    let m: usize = read(&mut stdin_lock, &mut buf, b'\n');
    let mut ev: Vec<Vec<usize>> = vec![vec![]; n];
    let mut dv = vec![0i32; n];
    for _ in 0..m {
        let x: usize = read(&mut stdin_lock, &mut buf, b' ');
        let y: usize = read(&mut stdin_lock, &mut buf, b'\n');
        ev[x - 1].push(y - 1);
        dv[y - 1] += 1;
    }

    let mut q = VecDeque::new();
    for i in 0..n {
        if dv[i] == 0 {
            q.push_back(i);
        }
    }

    let mut nv = vec![0i32; n];
    while !q.is_empty() {
        let i = q.pop_front().unwrap();
        for &e in ev[i].iter() {
            dv[e] -= 1;
            if dv[e] == 0 {
                q.push_back(e);
                nv[e] = max(nv[e], nv[i] + 1);
            }
        }
    }
    println!("{}", nv.iter().max().unwrap());
}
