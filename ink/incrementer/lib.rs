#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod incrementer {

    #[ink(storage)]
    pub struct Incrementer {
        value: i32,
        my_value: ink_storage::collections::HashMap<AccountId, i32>,
    }

    impl Incrementer {
        #[ink(constructor)]
        pub fn new(init_value: i32) -> Self {
            Self {
                value: init_value,
                my_value: ink_storage::collections::HashMap::new(),
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self {
                value: 0,
                my_value: Default::default(),
            }
        }

        #[ink(message)]
        pub fn get(&self) -> i32 {
            self.value
        }

        #[ink(message)]
        pub fn inc(&mut self, add_value: i32) {
            self.value += add_value;
        }

        #[ink(message)]
        pub fn get_mine(&self) -> i32 {
            let caller = self.env().caller();
            self.my_value_or_zero(&caller)
        }

        #[ink(message)]
        pub fn inc_mine(&mut self, add_value: i32) {
            let caller = self.env().caller();
            let caller_value = self.my_value_or_zero(&caller);
            self.my_value.insert(caller, caller_value + add_value);
        }

        fn my_value_or_zero(&self, of: &AccountId) -> i32 {
            *self.my_value.get(of).unwrap_or(&0)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        // Alias `ink_lang` so we can use `ink::test`.
        use ink_lang as ink;

        #[test]
        fn default_works() {
            let increment = Incrementer::default();
            assert_eq!(increment.get(), 0);
        }

        #[test]
        fn it_works() {
            let mut increment = Incrementer::new(42);
            assert_eq!(increment.get(), 42);
            increment.inc(10);
            assert_eq!(increment.get(), 52);
            increment.inc(7);
            assert_eq!(increment.get(), 59);

        }

        #[ink::test]
        fn my_value_works() {
            let mut contract = Incrementer::new(11);
            assert_eq!(contract.get(), 11);
            assert_eq!(contract.get_mine(), 0);
            contract.inc_mine(5);
            assert_eq!(contract.get_mine(), 5);
            contract.inc_mine(10);
            assert_eq!(contract.get_mine(), 15);
        }
    }
}
