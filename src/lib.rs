use scrypto::prelude::*;

#[derive(ScryptoSbor, NonFungibleData)]
pub struct BadgeData {
}

#[blueprint]
mod hello {
    struct Hello {
        badge_resource_manager: ResourceManager
    }

    impl Hello {
        // Implement the functions and methods which will manage those resources and data

        // This is a function, and can be called directly on the blueprint once deployed
        pub fn instantiate_hello() -> (Global<Hello>, Bucket) {
            // Create a new token called "HelloToken," with a fixed supply of 1000, and put that supply into a bucket
            
            let (address_reservation, component_address) = Runtime::allocate_component_address(Hello::blueprint_id());

            let badge_data = BadgeData {
            };

            let badge = ResourceBuilder::new_ruid_non_fungible::<BadgeData>(OwnerRole::None)
                .metadata(metadata! (
                    init {
                        "name" => "Badge", locked;
                    }
                ))
                .burn_roles(burn_roles! (
                    burner => rule!(require(global_caller(component_address)));
                    burner_updater => rule!(deny_all);
                ))
                .mint_initial_supply(vec![badge_data]);


                let component = Self {
                        badge_resource_manager: badge.resource_manager()
                    }
                .instantiate()
                .prepare_to_globalize(OwnerRole::None)
                .with_address(address_reservation)
                .globalize();

            return (component, badge.into());
        }

        pub fn x_function(&self, badge_proof: Proof) {
            let checked_proof: CheckedProof = badge_proof.check(self.badge_resource_manager.address());
            let non_fungible: NonFungible<BadgeData> = checked_proof.as_non_fungible().non_fungible();
            // do something with the non_fungible
        }
        
        pub fn y_function(&self, badge_bucket: Bucket) {
            let proof = badge_bucket.create_proof_of_all();
            self.x_function(proof);
            badge_bucket.burn();
        }
    }
}
