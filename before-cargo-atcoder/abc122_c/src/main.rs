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
    let s: String = read(&mut stdin_lock, &mut buf, b'\n');
    let mut lrv = vec![(0usize, 0usize); q];
    for i in 0..q {
        lrv[i].0 = read(&mut stdin_lock, &mut buf, b' ');
        lrv[i].1 = read(&mut stdin_lock, &mut buf, b'\n');
    }

    let mut tbl = vec![0usize; n];
    let cv: Vec<char> = s.chars().collect();
    for i in 1..n {
        tbl[i] = tbl[i - 1]
            + if cv[i - 1] == 'A' && cv[i] == 'C' {
                1
            } else {
                0
            };
    }

    for i in 0..q {
        println!("{}", tbl[lrv[i].1 - 1] - tbl[lrv[i].0 - 1]);
    }
}
