use crate::soulstore::soulstore::Soulstore;
use scrypto::prelude::*;

// Soulstores should be created via this blueprint. This allows each soulstore component address
// to be stored in a global key-value store, which can be used to look up the address of a soulstore.
// The soulstore_creator blueprint also keeps track of the number of soulstores created.

#[blueprint]
mod soulstore_creator {
    struct Soulapi {
        soulstore_addresses: KeyValueStore<NonFungibleGlobalId, ComponentAddress>,
        soulstores_created: i64,
    }

    impl Soulapi {
        pub fn instantiate_soul_api() -> Global<Soulapi> {
            Self {
                soulstore_addresses: KeyValueStore::new(),
                soulstores_created: 0,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .globalize()
        }

        pub fn create_soulstore(&mut self, global_id: NonFungibleGlobalId) {
            let soulstore_exists = self.soulstore_addresses.get(&global_id).is_some();

            assert!(!soulstore_exists, "Soulstore already exists for this NFT");

            let address = Soulstore::instantiate_soulstore(global_id.clone());

            self.soulstore_addresses.insert(global_id, address);

            self.soulstores_created += 1;
        }
    }
}
