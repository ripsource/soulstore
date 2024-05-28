use scrypto::prelude::*; // #1

// A soulstore acts a proxy for a typical Radix account, replicating a subset of the functionality
// such as depositing and withdrawing resources. Auth rules are set for the withdraw and withdraw_non_fungibles
// methods to ensure that only the owner of the NFT can withdraw resources.

// Anyone can deposit resources into a soulstore and there is no way to block deposits. _placeholder_auth arguements
// are used for 'try_deposit' methods so that they can accept a 'None' argument in compliance with how the Radix
// wallet generates typical transfer manifests. However, they have no other effect.

#[blueprint]
mod soulstore {
    struct Soulstore {
        vaults: KeyValueStore<ResourceAddress, Vault>,
        global_id: NonFungibleGlobalId,
    }

    impl Soulstore {
        pub fn instantiate_soulstore(global_id: NonFungibleGlobalId) -> ComponentAddress {
            let (address_reservation, component_address) =
                Runtime::allocate_component_address(Soulstore::blueprint_id());

            Self {
                vaults: KeyValueStore::new(),
                global_id,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .with_address(address_reservation)
            .globalize();

            component_address
        }

        pub fn deposit(&mut self, single_bucket: Bucket) {
            assert!(!single_bucket.is_empty(), "Cannot deposit an empty bucket");

            if single_bucket.resource_address() == self.global_id.resource_address() {
                assert!(
                    !single_bucket
                        .as_non_fungible()
                        .contains_non_fungible(self.global_id.local_id()),
                    "Depositing the soulstore's own NFT is not allowed. Nice try though."
                )
            }

            let resource_address = single_bucket.resource_address();

            let vault_exists = self.vaults.get(&resource_address).is_some();

            if vault_exists {
                let mut vault = self.vaults.get_mut(&resource_address).unwrap();
                vault.put(single_bucket);
            } else {
                self.vaults
                    .insert(resource_address.clone(), Vault::with_bucket(single_bucket));
            }
        }

        pub fn deposit_batch(&mut self, buckets: Vec<Bucket>) {
            for single_bucket in buckets {
                assert!(!single_bucket.is_empty(), "Cannot deposit an empty bucket");

                let resource_address = single_bucket.resource_address();

                if resource_address == self.global_id.resource_address() {
                    assert!(
                        !single_bucket
                            .as_non_fungible()
                            .contains_non_fungible(self.global_id.local_id()),
                        "Depositing the soulstore's own NFT is not allowed. Nice try though."
                    )
                }

                if self.vaults.get(&resource_address).is_some() {
                    let mut vault = self.vaults.get_mut(&resource_address).unwrap();
                    vault.put(single_bucket);
                } else {
                    self.vaults
                        .insert(resource_address, Vault::with_bucket(single_bucket));
                }
            }
        }

        pub fn try_deposit_or_abort(
            &mut self,
            single_bucket: Bucket,
            _placeholder_auth: Option<ResourceOrNonFungible>,
        ) {
            assert!(!single_bucket.is_empty(), "Cannot deposit an empty bucket");

            let resource_address = single_bucket.resource_address();

            if resource_address == self.global_id.resource_address() {
                assert!(
                    !single_bucket
                        .as_non_fungible()
                        .contains_non_fungible(self.global_id.local_id()),
                    "Depositing the soulstore's own NFT is not allowed. Nice try though."
                )
            }

            let vault_exists = self.vaults.get(&resource_address).is_some();

            if vault_exists {
                let mut vault = self.vaults.get_mut(&resource_address).unwrap();
                vault.put(single_bucket);
            } else {
                self.vaults
                    .insert(resource_address, Vault::with_bucket(single_bucket));
            }
        }

        pub fn try_deposit_or_refund(
            &mut self,
            single_bucket: Bucket,
            _placeholder_auth: Option<ResourceOrNonFungible>,
        ) {
            assert!(!single_bucket.is_empty(), "Cannot deposit an empty bucket");

            let resource_address = single_bucket.resource_address();

            if resource_address == self.global_id.resource_address() {
                assert!(
                    !single_bucket
                        .as_non_fungible()
                        .contains_non_fungible(self.global_id.local_id()),
                    "Depositing the soulstore's own NFT is not allowed. Nice try though."
                )
            }

            let vault_exists = self.vaults.get(&resource_address).is_some();

            if vault_exists {
                let mut vault = self.vaults.get_mut(&resource_address).unwrap();
                vault.put(single_bucket);
            } else {
                self.vaults
                    .insert(resource_address, Vault::with_bucket(single_bucket));
            }
        }

        pub fn try_deposit_batch_or_abort(
            &mut self,
            buckets: Vec<Bucket>,
            _placeholder_auth: Option<ResourceOrNonFungible>,
        ) {
            for single_bucket in buckets {
                assert!(!single_bucket.is_empty(), "Cannot deposit an empty bucket");

                let resource_address = single_bucket.resource_address();

                if resource_address == self.global_id.resource_address() {
                    assert!(
                        !single_bucket
                            .as_non_fungible()
                            .contains_non_fungible(self.global_id.local_id()),
                        "Depositing the soulstore's own NFT is not allowed. Nice try though."
                    )
                }

                if self.vaults.get(&resource_address).is_some() {
                    let mut vault = self.vaults.get_mut(&resource_address).unwrap();
                    vault.put(single_bucket);
                } else {
                    self.vaults
                        .insert(resource_address, Vault::with_bucket(single_bucket));
                }
            }
        }

        pub fn try_deposit_batch_or_refund(
            &mut self,
            buckets: Vec<Bucket>,
            _placeholder_auth: Option<ResourceOrNonFungible>,
        ) {
            for single_bucket in buckets {
                assert!(!single_bucket.is_empty(), "Cannot deposit an empty bucket");

                let resource_address = single_bucket.resource_address();

                if resource_address == self.global_id.resource_address() {
                    assert!(
                        !single_bucket
                            .as_non_fungible()
                            .contains_non_fungible(self.global_id.local_id()),
                        "Depositing the soulstore's own NFT is not allowed. Nice try though."
                    )
                }

                if self.vaults.get(&resource_address).is_some() {
                    let mut vault = self.vaults.get_mut(&resource_address).unwrap();
                    vault.put(single_bucket);
                } else {
                    self.vaults
                        .insert(resource_address, Vault::with_bucket(single_bucket));
                }
            }
        }

        pub fn withdraw(
            &mut self,
            resource_address: ResourceAddress,
            amount: Decimal,
            proof: Proof,
        ) -> Bucket {
            assert!(amount > Decimal::zero(), "Cannot withdraw an empty bucket");
            assert!(
                self.vaults.get(&resource_address).is_some(),
                "Vault does not exist"
            );

            let checked_proof = proof.check_with_message(
                self.global_id.resource_address(),
                "Incorrect ResourceAddress",
            );

            assert!(
                checked_proof
                    .as_non_fungible()
                    .contains_non_fungible(self.global_id.local_id()),
                "Incorrect NonFungibleLocalId"
            );

            let mut vault = self.vaults.get_mut(&resource_address).unwrap();
            vault.take(amount)
        }

        pub fn withdraw_non_fungibles(
            &mut self,
            resource_address: ResourceAddress,
            non_fungible_local_ids: IndexSet<NonFungibleLocalId>,
            proof: Proof,
        ) -> NonFungibleBucket {
            assert!(
                self.vaults.get(&resource_address).is_some(),
                "Vault does not exist"
            );

            let checked_proof = proof.check_with_message(
                self.global_id.resource_address(),
                "Incorrect Proof of Access: ResourceAddress",
            );

            assert!(
                checked_proof
                    .as_non_fungible()
                    .contains_non_fungible(self.global_id.local_id()),
                "Incorrect Proof of Access: NonFungibleLocalId"
            );

            let vault = self.vaults.get_mut(&resource_address).unwrap();

            vault
                .as_non_fungible()
                .take_non_fungibles(&non_fungible_local_ids)
        }
    }
}
