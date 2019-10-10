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
    let mut cv = vec![vec![0i32; 3]; 3];
    for y in 0..3 {
        for x in 0..3 - 1 {
            cv[y][x] = read(&mut stdin_lock, &mut buf, b' ');
        }
        cv[y][3 - 1] = read(&mut stdin_lock, &mut buf, b'\n');
    }

    let mut ans = false;
    for a1 in -100..100 + 1 {
        let b1 = cv[0][0] - a1;
        let b2 = cv[0][1] - a1;
        let b3 = cv[0][2] - a1;
        let a2 = cv[1][0] - b1;
        let a3 = cv[2][0] - b1;
        if a2 == cv[1][1] - b2 && a2 == cv[1][2] - b3 && a3 == cv[2][1] - b2 && a3 == cv[2][2] - b3
        {
            ans = true;
            break;
        }
    }
    println!("{}", if ans { "Yes" } else { "No" });
}
