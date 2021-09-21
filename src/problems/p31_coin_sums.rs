pub fn run() {
    let result = dfs(200, 200);
    println!("2$ can be made in {} ways", result);
}

fn dfs(amount: i32, last: i32) -> usize {
    if amount == 0 {
        return 1;
    }
    let coins_in_p = [1, 2, 5, 10, 20, 50, 100, 200];
    let mut result = 0;

    for coin in coins_in_p {
        let new_amount = amount - coin;
        if new_amount >= 0 && coin <= last {
            result += dfs(new_amount, coin);
        }
    }
    result
}
