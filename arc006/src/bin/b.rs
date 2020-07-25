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

    let _: usize = read(&mut stdin_lock, &mut buf, b' ');
    let n_l: usize = read(&mut stdin_lock, &mut buf, b'\n');
    let mut l: Vec<Vec<char>> = Vec::new();
    for _ in 0..n_l {
        let l_i: String = read(&mut stdin_lock, &mut buf, b'\n');
        l.push(l_i.chars().collect::<Vec<char>>());
    }
    let o: String = read(&mut stdin_lock, &mut buf, b'o');
    let o: usize = o.len(); // Usize1

    let mut i = o;
    for l_i in l.iter().rev() {
        if i > 1 && l_i[i - 1] == '-' {
            i = i - 2;
        } else if i < l_i.len() - 1 && l_i[i + 1] == '-' {
            i = i + 2;
        }
    }
    let ans = (i + 2) / 2;
    println!("{}", ans);
}
