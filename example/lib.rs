#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use ink_prelude::vec::Vec;
pub use nftmart_contract::*;

#[ink::contract(env = CustomEnvironment)]
mod contract_demo {
    use super::*;

    #[cfg(not(feature = "ink-as-dependency"))]
    #[ink(storage)]
    pub struct ContractDemo {
        value: [u8; 32],
    }

    #[ink(event)]
    pub struct RandomUpdated {
        #[ink(topic)]
        new: [u8; 32],
    }

    #[ink(event)]
    pub struct CreateClassFromContract {
        #[ink(topic)]
        owner: AccountId,
        class_id: ClassId,
    }

    impl ContractDemo {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { value: Default::default() }
        }

        /// hello 1
        #[ink(message)]
        pub fn tokens(&self, class_id: ClassId, token_id: TokenId) -> (Metadata, Quantity, BlockNumber) {
            let info: Option<ContractTokenInfo<_, _, _, _, _>> = self.env().extension().tokens(class_id, token_id);
            let info = info.unwrap_or_default();
            (info.metadata, info.quantity, info.data.create_block)
        }

        #[ink(message)]
        pub fn create_class(
            &mut self,
            creator: AccountId,
            metadata: Metadata,
            name: Chars,
            description: Chars,
            properties: u8,
        ) -> Result<(), NFTMartErr> {
            let (owner, class_id) = self.env().extension().create_class(&creator, metadata, name, description, properties)?;
            self.env().emit_event(CreateClassFromContract { owner, class_id });
            Ok(())
        }

        #[ink(message)]
        pub fn create_class_by_caller(
            &mut self,
            metadata: Metadata,
            name: Chars,
            description: Chars,
            properties: u8,
        ) -> Result<(), NFTMartErr> {
            let (owner, class_id) = self.env().extension().create_class_by_caller(metadata, name, description, properties)?;
            self.env().emit_event(CreateClassFromContract { owner, class_id });
            Ok(())
        }

        #[ink(message)]
        pub fn create_class_by_contract(
            &mut self,
            metadata: Metadata,
            name: Chars,
            description: Chars,
            properties: u8,
        ) -> Result<(), NFTMartErr> {
            let (owner, class_id) = self.env().extension().create_class_by_contract(metadata, name, description, properties)?;
            self.env().emit_event(CreateClassFromContract { owner, class_id });
            Ok(())
        }

        #[ink(message)]
        pub fn mint_nft(
            &mut self,
            creator: AccountId,
            to: AccountId,
            class_id: ClassId,
            metadata: Metadata,
            quantity: Quantity,
            charge_royalty: Option<bool>,
        ) -> Result<(), NFTMartErr> {
            let (_class_owner, _beneficiary, _class_id, _token_id, _quantity) = self.env().extension().proxy_mint(
                &creator,
                &to,
                class_id,
                metadata,
                quantity,
                charge_royalty,
            )?;
            Ok(())
        }

        #[ink(message)]
        pub fn mint_nft_by_caller(
            &mut self,
            to: AccountId,
            class_id: ClassId,
            metadata: Metadata,
            quantity: Quantity,
            charge_royalty: Option<bool>,
        ) -> Result<(), NFTMartErr> {
            let (_class_owner, _beneficiary, _class_id, _token_id, _quantity) = self.env().extension().proxy_mint_by_caller(
                &to,
                class_id,
                metadata,
                quantity,
                charge_royalty,
            )?;
            Ok(())
        }

        #[ink(message)]
        pub fn mint_nft_by_contract(
            &mut self,
            to: AccountId,
            class_id: ClassId,
            metadata: Metadata,
            quantity: Quantity,
            charge_royalty: Option<bool>,
        ) -> Result<(), NFTMartErr> {
            let (_class_owner, _beneficiary, _class_id, _token_id, _quantity) = self.env().extension().proxy_mint_by_contract(
                &to,
                class_id,
                metadata,
                quantity,
                charge_royalty,
            )?;
            Ok(())
        }

        #[ink(message)]
        pub fn transfer(
            &mut self,
            to: AccountId,
            class_id: ClassId,
            token_id: TokenId,
            quantity: Quantity,
        ) -> Result<(), NFTMartErr> {
            self.env().extension().transfer(&to, class_id, token_id, quantity)?;
            Ok(())
        }

        #[ink(message)]
        pub fn transfer_all(
            &mut self,
            to: AccountId,
            items: Vec<(ClassId, TokenId, Quantity)>,
        ) -> Result<(), NFTMartErr> {
            for (class_id, token_id, quantity) in items {
                self.env().extension().transfer(&to, class_id, token_id, quantity)?;
            }
            Ok(())
        }

        #[ink(message)]
        pub fn update(&mut self) -> Result<(), NFTMartErr> {
            let new_random = self.env().extension().fetch_random()?;
            self.value = new_random;
            self.env().emit_event(RandomUpdated { new: new_random });
            Ok(())
        }

        #[ink(message)]
        pub fn get(&self) -> [u8; 32] {
            self.value
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn new_works() {
            let _contract = ContractDemo::new();
        }
    }
}
