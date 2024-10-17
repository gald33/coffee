use scrypto::prelude::*;

#[blueprint]
mod coffee {
    enable_method_auth! {
        roles {
            // Define a role `price_updater` that cannot be updated by anyone.
            // TODO: later, setup a temporary owner that updates the price updater.
            price_updater => updatable_by: [];
        },
        methods {
            get_price => PUBLIC;
            set_price => restrict_to: [price_updater];
            buy => PUBLIC;
            sell => PUBLIC;
        }
    }

    struct Coffee {
        // Collateral tokens (XRD) are kept as collateral for minted coffee. Token type is set at instantiation.
        collateral_vault: FungibleVault,

        // Coffee are not held in the component, the resource address is kept instead
        token_manager: ResourceManager,

        // The median shop price of the most consumed coffee type in shops - in XRD
        price: Decimal,

        // the relative fee when buying or selling
        // the fee is kept to increase the collateral and guard against volatility
        friction: Decimal,

        // Only an authorized price updater with this badge can update the price
        price_updater_badge_manager: ResourceManager,
    }

    // Coffee coin for USA region
    impl Coffee {
        pub fn instantiate_coffee_usa(
            colleteral_resource_address: ResourceAddress,
            initial_price: Decimal,
            initial_friction: Decimal,
        ) -> (Global<Coffee>, FungibleBucket, FungibleBucket) {
            // Prepare the component virtual badge so it can mint and burn coffee
            let (address_reservation, component_address) = 
                Runtime::allocate_component_address(Coffee::blueprint_id());

            // Create the token
            let coffee: ResourceManager = ResourceBuilder::new_fungible(OwnerRole::None)
                .divisibility(DIVISIBILITY_MAXIMUM)
                .metadata(metadata! {
                    init {
                        "name" => "Coffee [USA]", locked;
                        "symbol" => "CF1", locked;
                        "region" => "USA", locked;
                    }
                })
                .mint_roles(mint_roles!(
                    // the component is the minter
                    minter => rule!(require(global_caller(component_address)));
                    minter_updater => rule!(deny_all);
                ))
                .burn_roles(burn_roles!(
                    // the component is the burner
                    burner => rule!(require(global_caller(component_address)));
                    burner_updater => rule!(deny_all);
                ))
                .create_with_no_initial_supply();

            // Create a price updater badge
            let price_updater_badge = ResourceBuilder::new_fungible(OwnerRole::None)
                .divisibility(DIVISIBILITY_NONE)
                .metadata(metadata!(
                    init {
                        "name" => "Coffee price updater badge [USA]", locked;
                        "region" => "USA", locked;
                    }
                ))
                .mint_initial_supply(1);

            // Create an owner badge
            // The owner will temporarily be responsible new functionality which will be phased out in the future
            let owner_badge = ResourceBuilder::new_fungible(OwnerRole::None)
                .divisibility(DIVISIBILITY_NONE)
                .metadata(metadata!(
                    init {
                        "name" => "Owner badge [USA]", locked;
                        "region" => "USA", locked;
                    }
                ))
                .mint_initial_supply(1);

            (
                Self {
                    collateral_vault: FungibleVault::new(colleteral_resource_address),
                    token_manager: coffee,
                    price: initial_price,
                    friction: initial_friction,
                    price_updater_badge_manager: price_updater_badge.resource_manager(),
                }
                .instantiate()
                .prepare_to_globalize(OwnerRole::None)
                .roles(roles!(
                    price_updater => rule!(require(price_updater_badge.resource_address()));
                ))
                // use allocated address which allowed using a virtual badge
                .with_address(address_reservation)
                .globalize(),
                price_updater_badge,
                owner_badge,
            )
        }

        // This is a method, because it needs a reference to self. Methods can only be called on components
        pub fn get_price(&self) -> Decimal {
            info!("Coffee price in collateral units is {}", self.price);
            self.price
        }

        // New price is to be updated at particular times by an oracle
        // Conditions and payments are to be implemented in the future
        pub fn set_price(&mut self, new_price: Decimal) {
            // Logic for updating the price (e.g., if access control checks pass)
            self.price = new_price;
        }

        // buy coffee in exchange to the collateral token which will be kept in the component vault
        // the friction is added to the price
        pub fn buy(&mut self, collateral_bucket: FungibleBucket) -> Bucket {
            assert_eq!(
                collateral_bucket.resource_address(),
                self.collateral_vault.resource_address(),
                "Input resource is not the correct collateral token"
            );
            let collateral_amount: Decimal = collateral_bucket.amount();
            self.collateral_vault.put(collateral_bucket);
            let buying_factor: Decimal = 1 + self.friction;
            let buying_price: Decimal = self.price * buying_factor;
            let coffee_amount: Decimal = collateral_amount / buying_price;
            self.token_manager.mint(coffee_amount)
        }

        // sell coffee in exchange for the colleteral            
        // the friction is added to the price

        pub fn sell(&mut self, coffee_bucket: FungibleBucket) -> FungibleBucket {
            assert_eq!(
                coffee_bucket.resource_address(),
                self.token_manager.address(),
                "Input resource is not the correct coffee token"
            );
            let coffee_amount: Decimal = coffee_bucket.amount();
            coffee_bucket.burn();
            let selling_factor: Decimal = 1 - self.friction;
            let selling_price: Decimal = self.price * selling_factor;
            let collateral_amount: Decimal = coffee_amount * selling_price;
            self.collateral_vault.take(collateral_amount)
        }
    }
}
