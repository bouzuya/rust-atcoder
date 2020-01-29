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
    let mut vv = vec![0usize; n];
    for i in 0..n - 1 {
        vv[i] = read(&mut stdin_lock, &mut buf, b' ');
    }
    vv[n - 1] = read(&mut stdin_lock, &mut buf, b'\n');

    let mut ov = vec![(0i32, 0i32); 100_000];
    let mut ev = vec![(0i32, 0i32); 100_000];
    for i in 0..n {
        ov[vv[i]].0 = vv[i] as i32;
        ev[vv[i]].0 = vv[i] as i32;
        if i % 2 == 0 {
            ov[vv[i]].1 += 1;
        } else {
            ev[vv[i]].1 += 1;
        }
    }

    ov.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
    ev.sort_by(|&(_, a), &(_, b)| b.cmp(&a));

    let m = n as i32;
    let ans = if ov[0].0 == ev[0].0 {
        m - (ov[0].1 + ev[1].1)
    } else {
        m - (ov[0].1 + ev[0].1)
    };
    println!("{}", ans);
}
