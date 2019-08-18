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
    let q: usize = read(&mut stdin_lock, &mut buf, b'\n');
    let mut vv = vec![Vec::new(); n + 1];
    for _ in 0..n - 1 {
        let a: usize = read(&mut stdin_lock, &mut buf, b' ');
        let b: usize = read(&mut stdin_lock, &mut buf, b'\n');
        vv[a].push(b);
    }
    let mut cv = vec![0i64; n + 1];
    for _ in 0..q {
        let p: usize = read(&mut stdin_lock, &mut buf, b' ');
        let x: i64 = read(&mut stdin_lock, &mut buf, b'\n');
        cv[p] += x;
    }

    let mut ans = vec![0i64; n + 1];
    let mut queue = VecDeque::new();
    ans[1] = cv[1];
    for &b in vv[1].iter() {
        let pair = (b, cv[1]);
        queue.push_back(pair);
    }
    while !queue.is_empty() {
        match queue.pop_front() {
            Some(pair) => {
                let (b, c) = pair;
                let s = c + cv[b];
                ans[b] = s;
                for &bc in vv[b].iter() {
                    queue.push_back((bc, s));
                }
            }
            None => unreachable!(),
        }
    }
    for i in 1..n {
        print!("{} ", ans[i]);
    }
    println!("{}", ans[n]);
}
