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
    let mut p = (0i32, 0i32, 0i32);
    for _ in 0..n {
        let pa = p.0;
        let pb = p.1;
        let pc = p.2;
        let a: i32 = read(&mut stdin_lock, &mut buf, b' ');
        let b: i32 = read(&mut stdin_lock, &mut buf, b' ');
        let c: i32 = read(&mut stdin_lock, &mut buf, b'\n');
        p.0 = a + max(pb, pc);
        p.1 = b + max(pa, pc);
        p.2 = c + max(pa, pb);
    }
    println!("{}", max(max(p.0, p.1), p.2));
}
