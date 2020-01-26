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
    use std::cmp::min;

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

    abv.sort();

    // 体力減少が x 以上のうち最も魔力の消耗が少ないものを選ぶ
    let mut tbl = vec![0usize; h + 1]; // tbl[hp] = min_mp;
    for i in 1..h + 1 {
        let mut min_mp = usize::max_value();
        for j in 0..n {
            let (a, b) = abv[j];
            min_mp = min(min_mp, if i > a { tbl[i - a] + b } else { b });
        }
        tbl[i] = min_mp;
    }

    println!("{}", tbl[h]);
}
