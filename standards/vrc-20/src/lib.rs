#![no_std]

extern crate alloc;

use alloc::{string::String, vec::Vec};
use core::{fmt::Debug, mem};
use utils::*;

/* Manually written code */

pub const PREFIX: Prefix = Prefix::new(1);

// #[standard::standard]
/// **Vara Request for Comment 20.**
///
/// The implemented standard for fungible tokens created using the Vara Network.
///
/// <Functionality, concept and details description>
pub trait Vrc20 {
    /* Events section */

    // #[event(index = 1)]
    /// Value was transferred.
    fn _transfer(from: Address, to: Address, value: U256);

    // #[event(index = 2)]
    /// Value was transferred.
    fn _approval(owner: Address, spender: Address, value: U256);

    /* Functions section */

    // #[func(index = 1)]
    /// Return name.
    fn name(&self) -> String;

    // #[func(index = 2)]
    /// Return symbol.
    fn symbol(&self) -> String;

    // #[func(index = 3)]
    /// Return decimals.
    fn decimals(&self) -> u8;

    // #[func(index = 4)]
    /// Return total supply.
    fn total_supply(&self) -> U256;

    // #[func(index = 5)]
    /// Return balance of owner.
    fn balance_of(&self, owner: Address) -> U256;

    // #[func(index = 6)]
    /// Transfer value to address.
    fn transfer(&mut self, to: Address, value: U256) -> bool;

    // #[func(index = 7)]
    /// Transfer value to address from specific sender.
    fn transfer_from(&mut self, from: Address, to: Address, value: U256) -> bool;

    // #[func(index = 8)]
    /// Approve token usage.
    fn approve(&mut self, spender: Address, value: U256) -> bool;

    // #[func(index = 9)]
    /// Return allowance.
    fn allowance(&self, owner: Address, spender: Address) -> U256;
}

/* Expanded code */

pub struct Request(Vec<u8>);

impl Request {
    /// Request to return name.
    pub fn name() -> Self {
        Self(crate::PREFIX.encode_with_idx(1))
    }

    /// Request to return symbol.
    pub fn symbol() -> Self {
        Self(crate::PREFIX.encode_with_idx(2))
    }

    /// Request to return decimals.
    pub fn decimals() -> Self {
        Self(crate::PREFIX.encode_with_idx(3))
    }

    /// Request to return total supply.
    pub fn total_supply() -> Self {
        Self(crate::PREFIX.encode_with_idx(4))
    }

    /// Request to return balance of owner.
    pub fn balance_of(owner: Address) -> Self {
        Self(crate::PREFIX.encode_with_idx_and_data(5, encode(owner)))
    }

    /// Request to transfer value to address.
    pub fn transfer(to: Address, value: U256) -> Self {
        Self(crate::PREFIX.encode_with_idx_and_data(6, encode((to, value))))
    }

    /// Request to transfer value to address from specific sender.
    pub fn transfer_from(from: Address, to: Address, value: U256) -> Self {
        Self(crate::PREFIX.encode_with_idx_and_data(7, encode((from, to, value))))
    }

    /// Request to approve token usage.
    pub fn approve(spender: Address, value: U256) -> Self {
        Self(crate::PREFIX.encode_with_idx_and_data(8, encode((spender, value))))
    }

    /// Request to return allowance.
    pub fn allowance(owner: Address, spender: Address) -> Self {
        Self(crate::PREFIX.encode_with_idx_and_data(9, encode((owner, spender))))
    }
}

impl Debug for Request {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        /*
            May be implemented in more specific way:
            "Request::name()"
            "Request::transfer(to: 0x12..34, value: 1_000_000)"
        */

        const IDX: usize = mem::size_of::<u128>();
        const PAYLOAD: usize = IDX + 1;

        match self.0[IDX] {
            1 => write!(f, "Name"),
            2 => write!(f, "Symbol"),
            3 => write!(f, "Decimals"),
            4 => write!(f, "TotalSupply"),
            5 => {
                let owner: Address = decode(&self.0[PAYLOAD..]);
                f.debug_struct("BalanceOf").field("owner", &owner).finish()
            }
            6 => {
                let (to, value): (Address, U256) = decode(&self.0[PAYLOAD..]);
                f.debug_struct("Transfer")
                    .field("to", &to)
                    .field("value", &value)
                    .finish()
            }
            7 => {
                let (from, to, value): (Address, Address, U256) = decode(&self.0[PAYLOAD..]);
                f.debug_struct("TransferFrom")
                    .field("from", &from)
                    .field("to", &to)
                    .field("value", &value)
                    .finish()
            }
            8 => {
                let (spender, value): (Address, U256) = decode(&self.0[PAYLOAD..]);
                f.debug_struct("Approve")
                    .field("spender", &spender)
                    .field("value", &value)
                    .finish()
            }
            9 => {
                let (owner, spender): (Address, Address) = decode(&self.0[PAYLOAD..]);
                f.debug_struct("Allowance")
                    .field("owner", &owner)
                    .field("spender", &spender)
                    .finish()
            }
            _ => Err(core::fmt::Error),
        }
    }
}

pub struct Response(Vec<u8>);

impl Response {
    /// Response to `fn name(..)`.
    pub fn name(name: String) -> Self {
        Self(crate::PREFIX.encode_with_idx_and_data(1, encode(name)))
    }

    /// Response to `fn symbol(..)`.
    pub fn symbol(symbol: String) -> Self {
        Self(crate::PREFIX.encode_with_idx_and_data(2, encode(symbol)))
    }

    /// Response to `fn decimals(..)`.
    pub fn decimals(decimals: u8) -> Self {
        Self(crate::PREFIX.encode_with_idx_and_data(3, encode(decimals)))
    }

    /// Response to `fn total_supply(..)`.
    pub fn total_supply(total_supply: U256) -> Self {
        Self(crate::PREFIX.encode_with_idx_and_data(4, encode(total_supply)))
    }

    /// Response to `fn balance_of(..)`.
    pub fn balance_of(balance: U256) -> Self {
        Self(crate::PREFIX.encode_with_idx_and_data(5, encode(balance)))
    }

    /// Response to `fn transfer(..)`.
    pub fn transfer(success: bool) -> Self {
        Self(crate::PREFIX.encode_with_idx_and_data(6, encode(success)))
    }

    /// Response to `fn transfer_from(..)`.
    pub fn transfer_from(success: bool) -> Self {
        Self(crate::PREFIX.encode_with_idx_and_data(7, encode(success)))
    }

    /// Response to `fn approve(..)`.
    pub fn approve(success: bool) -> Self {
        Self(crate::PREFIX.encode_with_idx_and_data(8, encode(success)))
    }

    /// Response to `fn allowance(..)`.
    pub fn allowance(balance: U256) -> Self {
        Self(crate::PREFIX.encode_with_idx_and_data(9, encode(balance)))
    }
}

impl Debug for Response {
    fn fmt(&self, _f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Implementation may be similar to `Request`
        unimplemented!()
    }
}

pub struct Event(Vec<u8>);

impl Event {
    /// Event that value was transferred.
    pub fn transfer(from: Address, to: Address, value: U256) -> Self {
        Self(crate::PREFIX.encode_with_idx_and_data(0, encode((1u8, from, to, value))))
    }

    /// Event that value was approved.
    pub fn approval(owner: Address, spender: Address, value: U256) -> Self {
        Self(crate::PREFIX.encode_with_idx_and_data(0, encode((2u8, owner, spender, value))))
    }
}

impl Debug for Event {
    fn fmt(&self, _f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Implementation may be similar to `Request` and `Response`.
        unimplemented!()
    }
}

/*
    Here may be some `gstd`-related apis to interact with such service:
    ```
        // .. //

        use vrc_20::Instance;

        #[gstd_async::main]
        async fn main() {
            // .. //

            let mut vrc_20 = Instance::new(vrc_20_implementor_address);

            // This fn does send_for_reply (with following unwraps) to implementor
            // sending vrc_20::Request::transfer(..) payload with given args,
            // expecting vrc_20::Response::transfer(..) result.
            assert!(vrc_20.transfer(to_address, value).await);

            assert_eq!(vrc_20.balance_of(to_address).await, value);

            // .. //
        }
    ```
*/

pub enum Error {
    WrongStandard,
    WrongCall,
    WrongArguments,
}

mod sealed {
    pub trait Sealed {}
}

pub trait Vrc20Processor: sealed::Sealed + Vrc20 {
    fn process(&mut self, request: Request) -> Result<Response, Error> {
        const IDX: usize = mem::size_of::<u128>();
        const PAYLOAD: usize = IDX + 1;

        if request.0[..IDX] != PREFIX.inner().to_le_bytes() {
            return Err(Error::WrongStandard);
        }

        match request.0[IDX] {
            // TODO: consider asserting request's emptiness (WrongArguments).
            1 => Ok(Response::name(self.name())),
            2 => Ok(Response::symbol(self.symbol())),
            // .. //
            9 => {
                let (owner, spender): (Address, Address) =
                    decode_fallible(&request.0[PAYLOAD..]).map_err(|_| Error::WrongArguments)?;

                Ok(Response::allowance(self.allowance(owner, spender)))
            }
            _ => Err(Error::WrongCall),
        }
    }
}

/* Utils mocking external deps for showcase */

pub mod utils {
    use super::*;

    pub type Address = [u8; 32];
    pub type U256 = [u8; 32];

    pub fn encode<T>(_data: T) -> &'static [u8] {
        b"place_holder"
    }

    pub fn decode<T: Default + Debug>(_buff: &[u8]) -> T {
        Default::default()
    }

    pub struct DecodeError;

    pub fn decode_fallible<T: Default + Debug>(_buff: &[u8]) -> Result<T, DecodeError> {
        Ok(Default::default())
    }

    pub struct Prefix(u128);

    impl Prefix {
        pub const fn new(prefix: u128) -> Self {
            Self(prefix)
        }

        pub const fn inner(self) -> u128 {
            self.0
        }

        pub fn encode(self) -> Vec<u8> {
            self.0.to_le_bytes().to_vec()
        }

        pub fn encode_with_idx(self, idx: u8 /* non zero u8 */) -> Vec<u8> {
            [self.0.to_le_bytes().as_ref(), &[idx]].concat()
        }

        pub fn encode_with_idx_and_data(
            self,
            idx: u8, /* non zero u8 */
            data: &[u8],
        ) -> Vec<u8> {
            [self.0.to_le_bytes().as_ref(), &[idx], data].concat()
        }
    }
}
