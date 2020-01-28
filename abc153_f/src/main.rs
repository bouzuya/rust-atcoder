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

    // input
    let n: usize = read(&mut stdin_lock, &mut buf, b' ');
    let d: usize = read(&mut stdin_lock, &mut buf, b' ');
    let a: usize = read(&mut stdin_lock, &mut buf, b'\n');
    let mut xhv = vec![(0usize, 0usize); n];
    for i in 0..n {
        xhv[i].0 = read(&mut stdin_lock, &mut buf, b' ');
        xhv[i].1 = read(&mut stdin_lock, &mut buf, b'\n');
    }

    xhv.sort();

    let mut ans = 0;
    let mut i = 0;
    while i < n {
        let (x, h) = xhv[i];
        let c = (h + a - 1) / a;
        let damage = c * a;
        let mut streak = true;
        let mut next_i = i + 1;
        for j in i + 1..xhv.len() {
            let (jx, jh) = xhv[j];
            if jx > x + 2 * d {
                break;
            }
            xhv[j].1 = jh.saturating_sub(damage);
            if streak && xhv[j].1 > 0 {
                streak = false;
                next_i = j;
            }
        }
        i = next_i;
        ans += c;
    }
    println!("{}", ans);
}
