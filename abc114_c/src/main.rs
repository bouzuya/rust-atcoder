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

fn is753(n: i32) -> bool {
    let mut has3 = false;
    let mut has5 = false;
    let mut has7 = false;
    let mut m = n;
    while m > 0 {
        match m % 10 {
            3 => has3 = true,
            5 => has5 = true,
            7 => has7 = true,
            _ => return false,
        }
        m /= 10;
    }
    has3 && has5 && has7
}

fn main() {
    let stdin = std::io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut buf: Vec<u8> = Vec::new();
    let n: i32 = read(&mut stdin_lock, &mut buf, b'\n');

    let mut v = vec![0];
    let mut ans = 0;
    let mut m = n;
    while m > 0 {
        v = v
            .clone()
            .into_iter()
            .flat_map(|i| {
                vec![3, 5, 7]
                    .iter()
                    .map(|&j| i * 10 + j)
                    .collect::<Vec<i32>>()
            })
            .collect();
        ans += v.iter().filter(|&&i| i <= n && is753(i)).count();
        m /= 10;
    }
    println!("{}", ans);
}
