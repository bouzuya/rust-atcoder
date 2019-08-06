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
    m: i32,
    x: i32,
    y: i32,
    z: i32,
) -> i32 {
    if i == n {
        return if x == 0 || y == 0 || z == 0 {
            a + b + c // inf
        } else {
            m + (x - a).abs() + (y - b).abs() + (z - c).abs()
        };
    }
    return vec![
        dfs(lv, n, a, b, c, i + 1, m, x, y, z),
        dfs(
            lv,
            n,
            a,
            b,
            c,
            i + 1,
            m + if x == 0 { 0 } else { 10 },
            x + lv[i],
            y,
            z,
        ),
        dfs(
            lv,
            n,
            a,
            b,
            c,
            i + 1,
            m + if y == 0 { 0 } else { 10 },
            x,
            y + lv[i],
            z,
        ),
        dfs(
            lv,
            n,
            a,
            b,
            c,
            i + 1,
            m + if z == 0 { 0 } else { 10 },
            x,
            y,
            z + lv[i],
        ),
    ]
    .into_iter()
    .min()
    .unwrap();
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

    let ans = dfs(&lv, n, a, b, c, 0, 0, 0, 0, 0);
    println!("{}", ans);
}
