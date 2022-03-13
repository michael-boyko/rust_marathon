mod wallet_mng;

fn main() {
    let wallet = wallet_mng::creat_wallet(10);
    println!("I've got {} septims in the wallet.", wallet.septims);

    let mut i: i32 = 0;
    let wallets = wallet_mng::creat_wallets(5);

    for mut item in wallets {
        item.septims = i * i;
        println!("{} wallet: {} septims.", i, item.septims);
        i += 1;
    }
}
