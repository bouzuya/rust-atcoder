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
    let s: i32 = read(&mut stdin_lock, &mut buf, b'\n');
    let mut av: Vec<i32> = vec![s];
    loop {
        let n = av[av.len() - 1];
        let a = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        av.push(a);
        if av.iter().filter(|i| **i == a).count() >= 2 {
            break;
        }
    }
    println!("{}", av.len());
}
