use std::cmp::min;

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
    let m: usize = read(&mut stdin_lock, &mut buf, b' ');
    let x: i32 = read(&mut stdin_lock, &mut buf, b'\n');
    let mut av = vec![0; n];
    for i in 0..m - 1 {
        av[i] = read(&mut stdin_lock, &mut buf, b' ');
    }
    av[m - 1] = read(&mut stdin_lock, &mut buf, b'\n');

    let mut cost1 = 0;
    let mut cost2 = 0;
    for i in 0..m {
        if x < av[i] {
            cost1 += 1;
        } else {
            cost2 += 1;
        }
    }
    let ans = min(cost1, cost2);
    println!("{}", ans);
}
