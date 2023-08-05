// WA
use std::io::StdinLock;

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

fn question(stdin_lock: &mut StdinLock, buf: &mut Vec<u8>, a: &[usize]) -> bool {
    let n = a.len();
    let mut output = String::new();
    output.push('?');
    output.push(' ');
    for (i, a_i) in a.iter().copied().enumerate() {
        output.push_str(&(a_i + 1).to_string());
        if i != n - 1 {
            output.push(' ');
        }
    }
    println!("{}", output);

    let t: i64 = read(stdin_lock, buf, b'\n');
    if t == -1 {
        unreachable!();
    } else if t == 0 {
        false
    } else if t == 1 {
        true
    } else {
        unreachable!()
    }
}

fn answer(a: &[usize]) {
    let n = a.len();
    let mut output = String::new();
    output.push('!');
    output.push(' ');
    for (i, a_i) in a.iter().copied().enumerate() {
        output.push_str(&a_i.to_string());
        if i != n - 1 {
            output.push(' ');
        }
    }
    println!("{}", output);
}

fn main() {
    let stdin = std::io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut buf: Vec<u8> = Vec::new();

    let nk: String = read(&mut stdin_lock, &mut buf, b'\n');
    let n: usize = nk.split(' ').collect::<Vec<&str>>()[0].parse().unwrap();
    let k: usize = nk.split(' ').collect::<Vec<&str>>()[1].parse().unwrap();

    if k == 1 {
        let mut ans = vec![];
        let mut q = vec![0];
        for i in 0..n {
            q.push(i);
            let is_odd = question(&mut stdin_lock, &mut buf, &q);
            q.pop();
            if is_odd {
                ans.push(1);
            } else {
                ans.push(0);
            }
        }
        answer(&ans);
    } else {
        // 1
        let mut q = (0..k).collect::<Vec<usize>>();
        let is_odd_0 = question(&mut stdin_lock, &mut buf, &q);

        // 1 + (n - k)
        let mut first = true;
        let mut is_odd_1 = false;
        let mut a = vec![2; n];
        a[0] = 0;
        for i in k..n {
            q[0] = i;
            let is_odd = question(&mut stdin_lock, &mut buf, &q);
            a[i] = if is_odd_0 == is_odd { a[0] } else { 1 - a[0] };
            if first {
                first = false;
                is_odd_1 = is_odd;
            }
        }

        let mut q = (0..k).collect::<Vec<usize>>();
        q.pop();
        q.pop();
        q.push(k);
        let mut is_odd_prev = is_odd_1;
        let mut first = true;
        for i in (1..k).rev() {
            q.push(i);
            let is_odd = question(&mut stdin_lock, &mut buf, &q);
            if first {
                first = false;
                a[i] = if is_odd_prev == is_odd {
                    a[k]
                } else {
                    1 - a[k]
                };
            } else {
                a[i] = if is_odd_prev == is_odd {
                    a[i + 1]
                } else {
                    1 - a[i + 1]
                };
            }
            q.pop();
            is_odd_prev = is_odd;
        }
        let ans = if is_odd_0 == (a[0..k].iter().copied().sum::<usize>() % 2 != 0) {
            a
        } else {
            a.iter().copied().map(|a_i| 1 - a_i).collect::<Vec<usize>>()
        };
        answer(&ans);
    }
}
