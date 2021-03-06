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
    let k: i32 = read(&mut stdin_lock, &mut buf, b' ');
    let q: usize = read(&mut stdin_lock, &mut buf, b'\n');
    let mut av = vec![0usize; q];
    for i in 0..q {
        av[i] = read(&mut stdin_lock, &mut buf, b'\n');
    }

    let mut cv = vec![0i32; n];
    for i in 0..q {
        cv[av[i] - 1] += 1;
    }

    for i in 0..n {
        println!(
            "{}",
            if (k + cv[i]) as usize > q {
                "Yes"
            } else {
                "No"
            }
        )
    }
}
