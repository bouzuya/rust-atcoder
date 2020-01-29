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

fn add357(lv: &Vec<i32>) -> Vec<i32> {
    return lv
        .iter()
        .flat_map(|&m| vec![m * 10 + 3, m * 10 + 5, m * 10 + 7])
        .collect();
}

fn is357(n: i32) -> bool {
    let mut m = n;
    let mut has3 = false;
    let mut has5 = false;
    let mut has7 = false;
    while m != 0 {
        match m % 10 {
            3 => has3 = true,
            5 => has5 = true,
            7 => has7 = true,
            _ => return false,
        }
        m /= 10;
    }
    return has3 && has5 && has7;
}

fn main() {
    let stdin = std::io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut buf: Vec<u8> = Vec::new();
    let n: i32 = read(&mut stdin_lock, &mut buf, b'\n');

    let mut lav = vec![];
    let mut lv = vec![0i32];
    for _ in 0..9 {
        lv = add357(&lv);
        lav.append(&mut lv.clone());
    }

    let ans = lav.iter().filter(|&&i| i <= n && is357(i)).count();
    println!("{}", ans);
}
