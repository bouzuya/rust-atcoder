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
    let s: String = read(&mut stdin_lock, &mut buf, b'\n');
    let mut cs: Vec<char> = s.chars().collect();
    let mut ans = true;
    if cs[0] != 'A' {
        ans = false;
    } else {
        cs[0] = 'a';
    }
    let mut c_count = 0;
    for i in 2..(cs.len() - 1) {
        if cs[i] == 'C' {
            cs[i] = 'c';
            c_count += 1;
        }
    }
    if c_count != 1 {
        ans = false;
    }
    if cs
        .into_iter()
        .filter(|c| !c.is_lowercase())
        .collect::<Vec<char>>()
        .len()
        != 0
    {
        ans = false;
    }
    println!("{}", if ans { "AC" } else { "WA" });
}
