use std::cmp::min;

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
    let s: String = read(&mut stdin_lock, &mut buf, b'\n');
    let cv: Vec<char> = s.chars().collect();

    let mut wv = vec![0i32; n];
    wv[0] = if cv[0] == 'W' { 1 } else { 0 };
    for i in 1..n {
        wv[i] = wv[i - 1] + if cv[i] == 'W' { 1 } else { 0 };
    }

    let mut ans = n as i32;
    for i in 0..n {
        // W -> E | E -> W
        let ec = if i == 0 { 0 } else { wv[i - 1] };
        let wc = (n - i) as i32 - 1 - (wv[n - 1] - wv[i]);
        let x = ec + wc;
        ans = min(ans, x);
    }
    println!("{}", ans);
}
