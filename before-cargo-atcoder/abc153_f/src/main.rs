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

fn upper_bound(v: &Vec<(usize, usize)>, x: &usize) -> usize {
    use std::cmp::Ordering;

    let mut l = 0;
    let mut h = v.len();
    while h > l {
        let m = l + (h - l) / 2;
        match v[m].0.cmp(x) {
            Ordering::Less | Ordering::Equal => {
                l = m + 1;
            }
            Ordering::Greater => {
                h = m;
            }
        }
    }
    l
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

    let mut counts = vec![0; xhv.len() + 1];
    let mut count = 0;
    for i in 0..xhv.len() {
        count -= counts[i];
        let (x, h) = xhv[i];
        if count * a < h {
            let c = ((h - (count * a)) + a - 1) / a;
            count += c;
            counts[upper_bound(&xhv, &(x + 2 * d))] += c;
        }
    }
    println!("{}", counts.iter().sum::<usize>());
}
