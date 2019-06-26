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
    let x1: i32 = read(&mut stdin_lock, &mut buf, b' ');
    let y1: i32 = read(&mut stdin_lock, &mut buf, b' ');
    let x2: i32 = read(&mut stdin_lock, &mut buf, b' ');
    let y2: i32 = read(&mut stdin_lock, &mut buf, b'\n');
    let dx = x2 - x1;
    let dy = y2 - y1;
    let x3 = x2 - dy;
    let y3 = y2 + dx;
    let x4 = x1 - dy;
    let y4 = y1 + dx;
    println!("{} {} {} {}", x3, y3, x4, y4);
}
