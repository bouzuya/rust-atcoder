use proconio::input;

fn main() {
    input! {
        s: (i64, i64),
        t: (i64, i64),
    };

    let mut c = vec![];
    let (dx, dy) = ((t.0 - s.0) as usize, (t.1 - s.1) as usize);
    c.append(&mut vec!['U'; dy]);
    c.append(&mut vec!['R'; dx]);
    c.append(&mut vec!['D'; dy]);
    c.append(&mut vec!['L'; dx]);
    c.push('L');
    c.append(&mut vec!['U'; dy + 1]);
    c.append(&mut vec!['R'; dx + 1]);
    c.push('D');
    c.push('R');
    c.append(&mut vec!['D'; dy + 1]);
    c.append(&mut vec!['L'; dx + 1]);
    c.push('U');
    let ans = c.iter().collect::<String>();
    println!("{}", ans);
}
