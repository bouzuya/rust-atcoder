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
    let c: String = read(&mut stdin_lock, &mut buf, b'\n');

    let mut s = vec![];
    for c_i in c.chars() {
        if c_i != ' ' {
            s.push(c_i);
            continue;
        }
        if let Some(p) = s.pop() {
            if p != ' ' {
                s.push(p);
            }
        }
        s.push(c_i);
    }
    let ans = s.iter().map(|&c| if c == ' ' { ',' } else { c }).collect::<String>();
    println!("{}", ans);
}
