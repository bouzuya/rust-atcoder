use std::cmp::Ordering;

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
    let mut ts = vec![("".to_owned(), 0, 0); n];
    for i in 0..n {
        ts[i].0 = read(&mut stdin_lock, &mut buf, b' ');
        ts[i].1 = read(&mut stdin_lock, &mut buf, b'\n');
        ts[i].2 = i + 1;
    }
    ts.sort_by(|a, b| match a.0.cmp(&b.0) {
        Ordering::Less => Ordering::Less,
        Ordering::Greater => Ordering::Greater,
        Ordering::Equal => b.1.cmp(&a.1),
    });
    for t in ts.iter() {
        println!("{}", t.2);
    }
}
