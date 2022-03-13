pub struct Wallet {
    pub septims: i32,
}

pub fn creat_wallets(n: i32) ->  Vec<Wallet> {
    let mut wallets = Vec::<Wallet>::new();
    let mut i: i32 = 0;

    while i < n {
        let wallet: Wallet = Wallet {septims: i};
        wallets.push(wallet);
        i += 1;
    }

    return wallets;
}
