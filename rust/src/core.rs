use hex::ToHex;
use std::str::FromStr;

use coins_bip32::path::DerivationPath;
use coins_bip39::{English, Mnemonic};
use ethers::core::{
    k256::ecdsa::SigningKey,
    utils::{secret_key_to_address, to_checksum},
};

#[derive(Debug)]
pub struct Wallet {
    address: String,
    private_key: String,
    mnemonic: String,
}

impl Wallet {
    /// Generate a random wallet
    pub fn random() -> eyre::Result<Wallet> {
        let word_count = 12usize;
        let derivation_path = DerivationPath::from_str("m/44'/60'/0'/0/0")?;

        // Generate a random mnemonic
        let mut rng = rand::thread_rng();
        let mnemonic = Mnemonic::<English>::new_with_count(&mut rng, word_count)?;

        // Derive the private key from the mnemonic
        let derived_priv_key = mnemonic.derive_key(derivation_path, None)?;
        let key: &coins_bip32::prelude::SigningKey = derived_priv_key.as_ref();
        let private_key = key.to_bytes();

        // Get the wallet address corresponding to the private key
        let signer = SigningKey::from_bytes(&private_key)?;
        let address = secret_key_to_address(&signer);

        Ok(Wallet {
            address: to_checksum(&address, None),
            private_key: private_key.encode_hex::<String>(),
            mnemonic: mnemonic.to_phrase()?,
        })
    }
}
