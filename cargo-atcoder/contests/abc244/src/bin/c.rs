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
    let mut used = vec![false; 2 * n + 1 + 1];
    while !used[0] {
        for i in 1..=2 * n + 1 {
            if used[i] {
                continue;
            }
            used[i] = true;
            println!("{}", i);
            break;
        }

        let x: usize = read(&mut stdin_lock, &mut buf, b'\n');
        used[x] = true;
    }
}
