use self::mod_u32::ModU32;
use proconio::input;
use proconio::marker::Usize1;

fn dfs(ans: &mut ModU32, n: usize, ess: &Vec<Vec<usize>>, u: usize, p: usize) -> usize {
    let mut t_us = 1; // u の部分木サイズ (頂点自身を含む)
    let mut t_vs = vec![]; // v の部分木サイズ
    for &v in ess[u].iter() {
        if v == p {
            continue;
        }
        let s = dfs(ans, n, ess, v, u);
        t_us += s;
        t_vs.push(s);
    }
    if p != n {
        t_vs.push(n - t_us); // dfs における親側の辺の部分木サイズ
    }
    *ans += t_vs.iter().fold(
        ModU32::new(2).pow((n - 1) as u32) - ModU32::new(1),
        |acc, &ts| acc - (ModU32::new(2).pow(ts as u32) - ModU32::new(1)),
    );
    t_us
}

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
    };
    let mut ess = vec![vec![]; n];
    for &(a, b) in ab.iter() {
        ess[a].push(b);
        ess[b].push(a);
    }

    // 答えは 穴あき度を x として x / 2^n
    // x は各頂点が次の 2 つをともに満たす場合の数
    // - その頂点が白である
    // - 辺のうち 2 つ以上が黒を含む部分木である
    // 余事象を求める
    // その頂点以外を自由に選ぶ場合 ( 2^(n - 1) ) から
    // 1 つの辺が黒を含む部分木である場合
    // = 部分木のうち少なくとも 1 つは黒でそれ以外を自由に選ぶ場合
    // ( 2^部分木サイズ - 1 ) を引く
    //
    let mut ans = ModU32::new(0);
    dfs(&mut ans, n, &ess, 0, n);
    ans = ans / ModU32::new(2).pow(n as u32);
    let ans: u32 = ans.into();
    println!("{:?}", ans);
}

mod mod_u32 {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct ModU32(u32);

    const MODP: u32 = 1_000_000_007;

    impl ModU32 {
        pub fn inv(self) -> Self {
            if self.0 == 0 {
                panic!()
            };
            self.pow(MODP - 2)
        }

        pub fn new(x: u64) -> Self {
            Self((x % (MODP as u64)) as u32)
        }

        pub fn pow(self, exp: u32) -> Self {
            let mut b: ModU32 = self;
            let mut a: ModU32 = Self::new(1);
            let mut x = exp;
            while x > 1 {
                if x & 1 == 1 {
                    a *= b;
                }
                x /= 2;
                b *= b;
            }
            if x == 1 {
                a *= b;
            }
            a
        }
    }

    impl From<i32> for ModU32 {
        fn from(x: i32) -> Self {
            ModU32::from(x as i64)
        }
    }

    impl From<i64> for ModU32 {
        fn from(x: i64) -> Self {
            let n = x % (MODP as i64);
            ModU32::new(if n.is_negative() { MODP as i64 + n } else { n } as u64)
        }
    }

    impl From<u32> for ModU32 {
        fn from(x: u32) -> Self {
            ModU32::from(x as u64)
        }
    }

    impl From<u64> for ModU32 {
        fn from(x: u64) -> Self {
            ModU32::new(x)
        }
    }

    impl From<usize> for ModU32 {
        fn from(x: usize) -> Self {
            ModU32::from(x as u64)
        }
    }

    impl Into<i32> for ModU32 {
        fn into(self) -> i32 {
            self.0 as i32
        }
    }

    impl Into<i64> for ModU32 {
        fn into(self) -> i64 {
            self.0 as i64
        }
    }

    impl Into<u32> for ModU32 {
        fn into(self) -> u32 {
            self.0
        }
    }

    impl Into<u64> for ModU32 {
        fn into(self) -> u64 {
            self.0 as u64
        }
    }

    impl Into<usize> for ModU32 {
        fn into(self) -> usize {
            self.0 as usize
        }
    }

    impl std::fmt::Display for ModU32 {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl std::ops::Add<ModU32> for ModU32 {
        type Output = ModU32;

        fn add(self, rhs: ModU32) -> Self::Output {
            (self.0 as u64 + rhs.0 as u64).into()
        }
    }

    impl std::ops::AddAssign<ModU32> for ModU32 {
        fn add_assign(&mut self, rhs: ModU32) {
            self.0 = (*self + rhs).0
        }
    }

    impl std::ops::Sub<ModU32> for ModU32 {
        type Output = ModU32;

        fn sub(self, rhs: ModU32) -> Self::Output {
            (if self.0 < rhs.0 {
                self.0 + (MODP - rhs.0)
            } else {
                self.0 - rhs.0
            })
            .into()
        }
    }

    impl std::ops::SubAssign<ModU32> for ModU32 {
        fn sub_assign(&mut self, rhs: ModU32) {
            self.0 = (*self - rhs).0
        }
    }

    impl std::ops::Mul<ModU32> for ModU32 {
        type Output = ModU32;

        fn mul(self, rhs: ModU32) -> Self::Output {
            Self::new((self.0 as u64) * (rhs.0 as u64))
        }
    }

    impl std::ops::MulAssign<ModU32> for ModU32 {
        fn mul_assign(&mut self, rhs: ModU32) {
            self.0 = (*self * rhs).0
        }
    }

    impl std::ops::Div<ModU32> for ModU32 {
        type Output = ModU32;

        fn div(self, rhs: ModU32) -> Self::Output {
            self * rhs.inv()
        }
    }

    impl std::ops::DivAssign<ModU32> for ModU32 {
        fn div_assign(&mut self, rhs: ModU32) {
            self.0 = (*self / rhs).0
        }
    }
}
