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
    let q: usize = read(&mut stdin_lock, &mut buf, b'\n');
    let mut qv: Vec<String> = Vec::new();
    for _ in 1..=q - 1 {
        let x: String = read(&mut stdin_lock, &mut buf, b'\n');
        qv.push(x);
    }
    {
        buf.clear();
        let mut x = String::new();
        std::io::BufRead::read_line(&mut stdin_lock, &mut x).unwrap();
        qv.push(x);
    }
    let mut d = 0; // 0 なら 先頭→末尾、 1 なら先頭←末尾
    let mut v = vec![vec![]; 2]; // 0 (先頭側) は逆順に格納する
    for s in qv.iter() {
        let cv: Vec<char> = s.chars().collect();
        if cv[0] == '1' {
            // 反転
            d = if d == 0 { 1 } else { 0 };
        } else {
            // 追加
            let f = cv[2];
            let c = cv[4];
            if f == '1' {
                // 先頭に追加
                v[d].push(c);
            } else {
                // 末尾に追加
                v[if d == 0 { 1 } else { 0 }].push(c);
            }
        }
    }
    let f: String = v[d].iter().rev().collect();
    let c: String = if d % 2 == 0 {
        s.chars().collect()
    } else {
        s.chars().rev().collect()
    };
    let b: String = v[if d == 0 { 1 } else { 0 }].iter().collect();
    print!("{}", f);
    print!("{}", c);
    println!("{}", b);
}
