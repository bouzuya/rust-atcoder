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
    let n: usize = read(&mut stdin_lock, &mut buf, b' ');
    let m: usize = read(&mut stdin_lock, &mut buf, b'\n');
    let mut psv = vec![(0usize, false); m];
    for i in 0..m {
        psv[i].0 = read(&mut stdin_lock, &mut buf, b' ');
        let s: String = read(&mut stdin_lock, &mut buf, b'\n');
        psv[i].1 = s == "AC";
    }

    let mut rv = vec![(0usize, false); n];
    for i in 0..psv.len() {
        let p = psv[i].0 - 1;
        let s = psv[i].1;
        if rv[p].1 {
            continue;
        } else {
            if s {
                rv[p].1 = s;
            } else {
                rv[p].0 += 1;
            }
        }
    }

    let mut wa = 0;
    let mut ac = 0;
    for i in 0..rv.len() {
        if rv[i].1 {
            wa += rv[i].0;
            ac += 1;
        }
    }
    println!("{} {}", ac, wa);
}
