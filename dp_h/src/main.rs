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
    let h: usize = read(&mut stdin_lock, &mut buf, b' ');
    let w: usize = read(&mut stdin_lock, &mut buf, b'\n');
    let mut g = vec![vec!['#'; w + 2]; h + 2];
    for i in 1..h + 1 {
        let line: String = read(&mut stdin_lock, &mut buf, b'\n');
        for (jz, c) in line.chars().enumerate() {
            g[i][jz + 1] = c;
        }
    }

    let mut tbl = vec![vec![0i32; w + 2]; h + 2];
    for i in 1..h + 1 {
        for j in 1..w + 1 {
            if i == 1 && j == 1 {
                tbl[i][j] = 1;
            } else {
                tbl[i][j] = (if g[i - 1][j] == '.' { tbl[i - 1][j] } else { 0 }
                    + if g[i][j - 1] == '.' { tbl[i][j - 1] } else { 0 })
                    % 1_000_000_007;
            }
        }
    }
    println!("{}", tbl[h][w]);
}
