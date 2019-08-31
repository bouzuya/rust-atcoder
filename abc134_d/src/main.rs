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
    let mut av = vec![0i32; n];
    for i in 0..n - 1 {
        av[i] = read(&mut stdin_lock, &mut buf, b' ');
    }
    av[n - 1] = read(&mut stdin_lock, &mut buf, b'\n');

    let mut cv = vec![0i32; n];
    for i in 0..n {
        let j = n - i - 1;
        let x = j + 1;

        let mut index = 2;
        let mut count = 0;
        while index * x <= n {
            count += cv[index * x - 1];
            index += 1;
        }

        if (count % 2) != (av[j] % 2) {
            cv[j] = 1;
        }
    }

    let mut ans = Vec::new();
    for i in 0..n {
        if cv[i] > 0 {
            ans.push(i + 1);
        }
    }

    println!("{}", ans.len());
    if ans.len() > 0 {
        let mut first = true;
        for i in 0..ans.len() {
            if first {
                first = false;
                print!("{}", ans[i]);
            } else {
                print!(" {}", ans[i]);
            }
        }
        println!();
    }
}
