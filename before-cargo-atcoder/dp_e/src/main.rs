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
    let n: usize = read(&mut stdin_lock, &mut buf, b' ');
    let w: i64 = read(&mut stdin_lock, &mut buf, b'\n');
    let w_max = 10_000_000_000i64;
    let v_max = 1_000 * n;
    let mut ans = vec![vec![w_max; v_max + 1]; n + 1];
    ans[0][0] = 0;
    for i in 0..n {
        let wi: i64 = read(&mut stdin_lock, &mut buf, b' ');
        let vi: usize = read(&mut stdin_lock, &mut buf, b'\n');
        for j in 0..v_max + 1 {
            ans[i + 1][j] = min(ans[i][j], if j < vi { wi } else { ans[i][j - vi] + wi });
        }
    }
    let mut ans_v = 0;
    for j in 0..v_max + 1 {
        if ans[n][j] <= w {
            ans_v = j;
        }
    }
    println!("{}", ans_v);
}
