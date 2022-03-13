pub struct Wallet {
    pub septims: i32,
}

pub fn creat_wallet(n: i32) -> Wallet {
    Wallet {septims: n}
}

pub fn creat_wallets(mut n: i32) ->  Vec<Wallet> {
    let mut wallets = Vec::<Wallet>::new();

    while 0 < n {
        wallets.push(creat_wallet(0));
        n -= 1;
    }

    return wallets;
}
