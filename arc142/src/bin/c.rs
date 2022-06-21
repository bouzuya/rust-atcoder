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

    let inf = n + 1;
    let mut d_1 = vec![inf; n + 1];
    let mut d_2 = vec![inf; n + 1];
    let mut count3 = vec![];
    let mut min = inf;
    for i in 3..=n {
        println!("? 1 {}", i);
        d_1[i] = read(&mut stdin_lock, &mut buf, b'\n');
        println!("? 2 {}", i);
        d_2[i] = read(&mut stdin_lock, &mut buf, b'\n');
        let d_i = d_1[i] + d_2[i];
        min = min.min(d_i);
        if d_i == 3 {
            count3.push(i);
        }
    }
    if min != 3 {
        println!("! {}", min);
        return;
    }
    if count3.len() != 2 {
        println!("! 1");
        return;
    }

    let (x, y) = (count3[0], count3[1]);
    println!("? {} {}", x, y);
    let d_xy: usize = read(&mut stdin_lock, &mut buf, b'\n');
    println!("! {}", if d_xy == 1 { 3 } else { 1 });
}
