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
    let s1: String = read(&mut stdin_lock, &mut buf, b'\n');
    let s2: String = read(&mut stdin_lock, &mut buf, b'\n');
    let s3: String = read(&mut stdin_lock, &mut buf, b'\n');
    let l1: Vec<char> = s1.chars().collect();
    let l2: Vec<char> = s2.chars().collect();
    let l3: Vec<char> = s3.chars().collect();
    let ans: String = vec![l1[0], l2[1], l3[2]].into_iter().collect();
    println!("{}", ans);
}
