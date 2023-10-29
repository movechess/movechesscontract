#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod movechesscontract {
    use ink::storage::Mapping;

    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(
            Debug,
            PartialEq,
            Eq,
            scale_info::TypeInfo,
            ink::storage::traits::StorageLayout
        )
    )]
    
    pub struct Game {
        user_a: AccountId,
        user_b: AccountId,
        winner: AccountId,
        value: Balance,
        user_a_payable: bool,
        user_b_payable: bool,
    }

    #[ink(storage)]
    #[derive(Default)]
    pub struct Movechesscontract {
        counter: u32,
        games: Mapping<u32, Game>
    }



    impl Movechesscontract {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(counter: u32) -> Self {
            Self { counter: counter, games: Mapping::default() }
        }


        // fn zero_address() -> AccountId {
        //     [0u8; 32].into()
        // }

        #[ink(message, payable)]
        pub fn match_game(&mut self, _index: u32) -> Result<Game ,()> {
            let _caller = self.env().caller();

            if let Some(mut _game_data) = self.games.get(_index) {
                // game done (user_a or user_b is winner)
                if _game_data.winner != AccountId::from([0xFF as u8; 32]) {
                    return Ok(_game_data);
                }

                if _game_data.user_a_payable && _game_data.user_b_payable {
                    return Ok(_game_data);
                }

                // available game for a matching with b;
                // user_a is available but unkown user_b -> user_b send native token matching with user_a
                if _game_data.value == 10000000000000 
                    && _game_data.user_a != AccountId::from([0xFF as u8; 32]) 
                    && _game_data.user_a_payable == true 
                    && _game_data.user_b == AccountId::from([0xFF as u8; 32]) 
                    && _game_data.user_b_payable == false {
                        // let _contract_balance = Self::env().balance();
                        let _amount = Self::env().transferred_value();
                        if _amount == 10000000000000 {
                            _game_data.user_b = _caller;
                            _game_data.user_b_payable = true;
                        }
                        self.games
                            .insert(&_index, &_game_data);
                        return Ok(_game_data);
                }    
                return Ok(_game_data);
            } else {
                 // user_a create new match
                let mut _game = Game {
                    user_a: _caller,
                    user_b: AccountId::from([0xFF as u8; 32]),
                    winner: AccountId::from([0xFF as u8; 32]),
                    value: 10000000000000,
                    user_a_payable: false,
                    user_b_payable: false,
                };
                // user_a transfer 1000 token to contract
                // let _contract_balance = Self::env().balance();
                let _amount = Self::env().transferred_value();
                // let _current_balance = _contract_balance.checked_sub(1000).unwrap_or_default();
                if _amount == 10000000000000 {
                    _game.user_a_payable = true;
                    self.games.insert(self.counter, &_game);
                    self.counter += 1;
                    return Ok(_game);
                } 
                return Ok(_game);
                
            }
        
           
        }

        #[ink(message)] 
        pub fn get_counter(&self) -> u32 {
            return self.counter;
        }

        #[ink(message)] 
        pub fn get_game_info(&self, _index: u32) -> Option<Game> {
            return self.games.get(_index);
        }

        #[ink(message)] 
        pub fn update_winner (&mut self, _index: u32, winner_index: u32) -> bool {
            let _caller = self.env().caller();
            // if _caller == self.convert(String::from("5EXVePY8xnyfGKQjrbvQUgH9bdeXb5YszFqvhhXCnWYT6kBw")) {
                if let Some(mut _game_data) = self.games.get(_index) {
                    if _game_data.winner == AccountId::from([0xFF as u8; 32]){
                        if winner_index == 0 {
                            _game_data.winner = _game_data.user_a;
                            let transfer_result = Self::env().transfer(_game_data.user_a, _game_data.value*2);
                            if transfer_result.is_err() {
                                return false;
                            } else {
                                self.games.insert(&_index, &_game_data);
                                return true;
                            }
                        } else {
                            _game_data.winner = _game_data.user_b;
                            let transfer_result = Self::env().transfer(_game_data.user_b, _game_data.value*2);
                            if transfer_result.is_err() {
                                return false;
                            } else {
                                self.games.insert(&_index, &_game_data);
                                return true;
                            }
                        }
                    }
                }
            // }
            return false;
        } 

        #[ink(message)]
        pub fn convert(&mut self, s: ink::prelude::string::String) -> AccountId {
            let bytes = s.as_bytes();
            let account = AccountId::try_from(bytes).expect("You should handle this properly.");
            return account;
        }
    }

    

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        fn default_accounts(
        ) -> ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> {
            ink::env::test::default_accounts::<ink::env::DefaultEnvironment>()
        }
        fn set_account_balance(account: AccountId, balance: Balance) {
            ink::env::test::set_account_balance::<ink::env::DefaultEnvironment>(
                account, balance,
            );
        }
        fn get_account_balance(account: AccountId) -> Balance {
            ink::env::test::get_account_balance::<ink::env::DefaultEnvironment>(account)
                .expect("Cannot get account balance")
        }

        #[ink::test]
        fn test_matching_game() {
            let accounts = default_accounts();
            let initial_balance = 10_000;
            set_account_balance(accounts.alice, initial_balance);
            set_account_balance(accounts.bob, initial_balance);

            let mut _movechesscontract = Movechesscontract::new(0);
            assert_eq!(_movechesscontract.get_counter(), 0);

            assert_eq!(_movechesscontract.get_game_value(0) == 0, true);
            _movechesscontract.match_game(0);
            
            // movechesscontract.flip();
            // assert_eq!(movechesscontract.get(), true);
        }
    }

 
}
