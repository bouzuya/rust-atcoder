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
    let mut av = vec![0i32; n];
    for i in 0..n {
        av[i] = read(&mut stdin_lock, &mut buf, b'\n');
    }

    let mut lv = vec![0i32; n];
    for i in 1..n {
        lv[i] = max(lv[i - 1], av[i - 1]);
    }
    let mut rv = vec![0i32; n];
    for i in 1..n {
        let j = n - i - 1;
        rv[j] = max(rv[j + 1], av[j + 1]);
    }

    for i in 0..n {
        println!("{}", max(lv[i], rv[i]));
    }
}
