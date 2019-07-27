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

fn dfs(
    lv: &Vec<i32>,
    n: usize,
    a: i32,
    b: i32,
    c: i32,
    i: usize,
    ax: i32,
    bx: i32,
    cx: i32,
) -> i32 {
    if i == n {
        return if ax == 0 || bx == 0 || cx == 0 {
            a + b + c
        } else {
            (ax - a).abs() - 10 + (bx - b).abs() - 10 + (cx - c).abs() - 10
        };
    }

    vec![
        dfs(lv, n, a, b, c, i + 1, ax, bx, cx),
        dfs(lv, n, a, b, c, i + 1, ax + lv[i], bx, cx) + 10,
        dfs(lv, n, a, b, c, i + 1, ax, bx + lv[i], cx) + 10,
        dfs(lv, n, a, b, c, i + 1, ax, bx, cx + lv[i]) + 10,
    ]
    .into_iter()
    .min()
    .unwrap()
}

fn main() {
    let stdin = std::io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut buf: Vec<u8> = Vec::new();
    let n: usize = read(&mut stdin_lock, &mut buf, b' ');
    let a: i32 = read(&mut stdin_lock, &mut buf, b' ');
    let b: i32 = read(&mut stdin_lock, &mut buf, b' ');
    let c: i32 = read(&mut stdin_lock, &mut buf, b'\n');
    let mut lv = vec![0i32; n];
    for i in 0..n {
        lv[i] = read(&mut stdin_lock, &mut buf, b'\n');
    }

    let ans = dfs(&lv, n, a, b, c, 0, 0, 0, 0);
    println!("{}", ans);
}
