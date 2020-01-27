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
    let h: usize = read(&mut stdin_lock, &mut buf, b' ');
    let n: usize = read(&mut stdin_lock, &mut buf, b'\n');
    let mut abv = vec![(0usize, 0usize); n];
    for i in 0..n {
        abv[i].0 = read(&mut stdin_lock, &mut buf, b' ');
        abv[i].1 = read(&mut stdin_lock, &mut buf, b'\n');
    }

    let mut tbl = vec![usize::max_value(); h + 1];
    tbl[0] = 0;
    for hp in 1..h + 1 {
        tbl[hp] = abv
            .iter()
            .map(|&(a, b)| if hp > a { tbl[hp - a] + b } else { b })
            .min()
            .unwrap();
    }

    println!("{}", tbl[h]);
}
