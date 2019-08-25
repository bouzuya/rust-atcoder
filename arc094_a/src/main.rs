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
    let a: i32 = read(&mut stdin_lock, &mut buf, b' ');
    let b: i32 = read(&mut stdin_lock, &mut buf, b' ');
    let c: i32 = read(&mut stdin_lock, &mut buf, b'\n');

    let mut xv = vec![a, b, c];
    xv.sort();
    let s1 = xv[2] - xv[1];
    let s2 = (xv[2] - (xv[0] + s1)) / 2;
    let s3 = if (xv[2] - (xv[0] + s1)) % 2 == 0 {
        0
    } else {
        2
    };
    let ans = s1 + s2 + s3;
    println!("{}", ans);
}
