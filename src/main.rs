use std::str::FromStr;
use bip39::{Language, Mnemonic};
use bitcoin::{Address, Network, PrivateKey, secp256k1};
use bitcoin::bip32::{DerivationPath, Xpriv};

fn main() {
    wif_to_p2tr();
    println!("============================");
    mnemonic_to_p2tr_address();
    println!("Hello, world!");
}

fn wif_to_p2tr(){
    let secp = secp256k1::Secp256k1::new();
    let priv_key = PrivateKey::from_wif("KxEEpvY3qbM4GfLjDAovg2yrszap6Gv7rupkHFwdPhLvzJXjTCQY").unwrap();
    let network = priv_key.network;
    let public_key = priv_key.public_key(&secp);
    let pub_key = priv_key.inner.x_only_public_key(&secp).0;
    let address_p2tr =Address::p2tr(&secp, pub_key, None, priv_key.network);
    let address_p2wpkh = Address::p2wpkh(&public_key, priv_key.network).unwrap();
    let address_legacy = Address::p2pkh(&public_key, priv_key.network);
    println!("network {}",network);
    println!("p2tr {}",address_p2tr);
    println!("p2wpkh {}",address_p2wpkh);
    println!("legacy {}",address_legacy);
}

fn mnemonic_to_p2tr_address()  {
    //生成 随机助记词
    let mnemonic = Mnemonic::generate(12).unwrap();
    println!("Mnemonic phrase: {}", mnemonic.to_string());
    // 替换为你的12个单词的助记词
    // 导入助记词
    // let mnemonic = Mnemonic::parse_in(Language::English, mnemonic_phrase).unwrap();
    // 生成种子和扩展私钥
    let seed = mnemonic.to_seed("");
    let secp = secp256k1::Secp256k1::new();
    let master_key = Xpriv::new_master(Network::Bitcoin, &seed).unwrap();
    // 派生m/86'/0'/0'/0路径下的私钥和公钥
    let path = DerivationPath::from_str("m/86'/0'/0'/0/0").unwrap();
    let derived_priv_key = master_key.derive_priv(&secp, &path).unwrap();
    let priv_key = derived_priv_key.to_priv();
    let wif =  priv_key.to_wif();
    let public_key = priv_key.public_key(&secp);
    let pub_key = priv_key.inner.x_only_public_key(&secp).0;
    let address =Address::p2tr(&secp, pub_key, None, priv_key.network);
    println!("Private key: {:?}", priv_key);
    println!("Public key: {:?}", public_key);
    println!("wif: {}", wif);
    println!("P2TR Address: {}", address);

    // Mnemonic phrase: lobster sweet actor group sleep equip nature know crew oppose author pen
    // Private key: PrivateKey { compressed: true, network: Bitcoin, inner: SecretKey(#e173ddc5109b4ca0) }
    // Public key: PublicKey { compressed: true, inner: PublicKey(9df20ff07872c69a672451ef56b80a9dccb56ba3fcc6a215b37844dc023c7f4c16413890ed76553f8fbf5a36f5f0c50017d20b92ec258237af5b6b56149a476e) }
    // wif: KxEEpvY3qbM4GfLjDAovg2yrszap6Gv7rupkHFwdPhLvzJXjTCQY
    // P2TR Address: bc1p6zmuqk9wvfljrvz5zwhhy3klyftywa4908nhesnsrfqq8aeyjz6sw26f0y
    // Hello, world!

}
