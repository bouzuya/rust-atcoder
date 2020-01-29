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
    let k: usize = read(&mut stdin_lock, &mut buf, b'\n');
    let mut hv = vec![0i32; n];
    for i in 0..n - 1 {
        hv[i] = read(&mut stdin_lock, &mut buf, b' ');
    }
    hv[n - 1] = read(&mut stdin_lock, &mut buf, b'\n');

    let mut cv = vec![0; n];
    for i in 1..n {
        cv[i] = cv[i - 1] + (hv[i] - hv[i - 1]).abs();
        for j in 2..k + 1 {
            if i < j {
                break;
            }
            cv[i] = min(cv[i], cv[i - j] + (hv[i] - hv[i - j]).abs());
        }
    }
    println!("{}", cv[n - 1]);
}
