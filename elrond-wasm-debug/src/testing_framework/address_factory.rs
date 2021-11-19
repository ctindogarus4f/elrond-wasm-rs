use elrond_wasm::types::Address;
use sha2::{Digest, Sha256};

const ADDRESS_LEN: usize = 32;
const SC_ADDR_LEADING_ZEROES: usize = 8;

pub struct AddressFactory {
    last_generated_address: [u8; ADDRESS_LEN],
}

impl AddressFactory {
    pub fn new() -> Self {
        Self {
            last_generated_address: [0u8; ADDRESS_LEN],
        }
    }

    pub fn new_address(&mut self) -> Address {
        Address::from(self.new_address_raw())
    }

    pub fn new_sc_address(&mut self) -> Address {
        let mut addr = self.new_address_raw();
        for i in 0..SC_ADDR_LEADING_ZEROES {
            addr[i] = 0;
        }

        Address::from(addr)
    }

    fn new_address_raw(&mut self) -> [u8; ADDRESS_LEN] {
        let mut hasher = Sha256::new();
        hasher.update(self.last_generated_address);
        let result: [u8; ADDRESS_LEN] = hasher.finalize().into();

        self.last_generated_address = result.clone();

        result
    }
}
