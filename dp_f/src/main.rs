use std::cmp::max;

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
    let s: String = read(&mut stdin_lock, &mut buf, b'\n');
    let t: String = read(&mut stdin_lock, &mut buf, b'\n');
    let sc: Vec<char> = s.chars().collect();
    let tc: Vec<char> = t.chars().collect();
    let mut tbl = vec![vec![0; tc.len() + 1]; sc.len() + 1];
    for i in 0..sc.len() {
        for j in 0..tc.len() {
            tbl[i + 1][j + 1] = max(
                tbl[i][j + 1],
                max(
                    tbl[i + 1][j],
                    tbl[i][j] + if sc[i] == tc[j] { 1 } else { 0 },
                ),
            );
        }
    }
    let mut ac: Vec<char> = Vec::new();
    let mut i: usize = sc.len();
    let mut j: usize = tc.len();
    while i > 0 && j > 0 {
        if tbl[i][j] == tbl[i - 1][j] {
            i -= 1;
        } else if tbl[i][j] == tbl[i][j - 1] {
            j -= 1;
        } else {
            ac.push(sc[i - 1]);
            i -= 1;
            j -= 1;
        }
    }
    println!("{}", ac.iter().rev().collect::<String>());
}
