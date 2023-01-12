// Copyright (C) 2019-2022 Aleo Systems Inc.

use rand_chacha::{rand_core::SeedableRng, ChaChaRng};
use serde::{Deserialize, Serialize};
use snarkvm_console::{
    account::{address::Address, private_key::PrivateKey, view_key::ViewKey, Error},
    network::Testnet3,
};

/// Aleo key tuple
#[derive(Clone, Serialize, Deserialize)]
pub struct AleoKeys {
    private_key: PrivateKey<Testnet3>,
    view_key: ViewKey<Testnet3>,
    address: Address<Testnet3>,
}

impl AleoKeys {
    /// Returns a new Aleo key tuple
    pub fn new() -> Result<Self, Error> {
        let mut rng = ChaChaRng::from_entropy();
        let private_key = PrivateKey::<Testnet3>::new(&mut rng)?;
        let view_key = ViewKey::<Testnet3>::try_from(&private_key)?;
        let address = Address::<Testnet3>::try_from(&view_key)?;
        Ok(Self {
            private_key,
            view_key,
            address,
        })
    }

    pub fn pretty_print(&self) {
        let private_key = self.private_key;
        let view_key = self.view_key;
        let address = self.address;
        let output = format!(
            " {:>12}  {private_key}\n {:>12}  {view_key}\n {:>12}  {address}",
            "Private Key",
            "View Key",
            "Address",
        );
        println!("{}", output);
    }
}

impl TryFrom<&PrivateKey<Testnet3>> for AleoKeys {
    type Error = Error;

    /// Derives the account tuple
    fn try_from(private_key: &PrivateKey<Testnet3>) -> Result<Self, Self::Error> {
        let view_key = ViewKey::<Testnet3>::try_from(private_key)?;
        let address = Address::<Testnet3>::try_from(&view_key)?;
        Ok(Self {
            private_key: private_key.clone(),
            view_key,
            address,
        })
    }
}
