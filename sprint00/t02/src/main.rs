mod wallet_mng;

fn main() {
    let size: i32 = 5;
    let wallets = wallet_mng::creat_wallets(size);

    for wallet in wallets {
        println!("{}", wallet.septims);
    }
}
