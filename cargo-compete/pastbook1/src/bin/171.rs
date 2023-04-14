use proconio::input;

fn f(n: usize, count: &mut usize, s: &mut Vec<char>) {
    if s.len() >= 3 {
        let x = s.iter().collect::<String>().parse::<usize>().unwrap();
        if x > n {
            return;
        } else if s.iter().any(|c| c == &'3')
            && s.iter().any(|c| c == &'5')
            && s.iter().any(|c| c == &'7')
        {
            *count += 1;
        }
    }

    for c in &['3', '5', '7'] {
        s.push(*c);
        f(n, count, s);
        s.pop();
    }
}

fn main() {
    input! {
        n: usize
    }

    let mut count = 0_usize;
    let mut s = vec![];
    f(n, &mut count, &mut s);
    println!("{}", count);
}
