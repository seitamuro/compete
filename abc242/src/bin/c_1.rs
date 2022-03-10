use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut dp = vec![vec![0u64; 10]; n+1];
    let MOD: u64 = 998244353;

    for i in 0..10 {
        dp[1][i] = 1;
    }

    // f(7???) = f(6??) + f(7??) + f(8??)
    for i in 2..=n {
        for j in 1..10 {
            if 1 < j && j < 9 {
                dp[i][j] = dp[i-1][j-1] + dp[i-1][j] + dp[i-1][j+1];
            } else if j == 1 {
                dp[i][j] = dp[i-1][1] + dp[i-1][2];
            } else {
                dp[i][j] = dp[i-1][9] + dp[i-1][8];
            }
            dp[i][j] %= MOD;
        }
    }

    let mut ans = 0;
    for i in 1..10 {
        ans += dp[n][i];
        ans %= MOD;
    }

    println!("{}", ans);
}