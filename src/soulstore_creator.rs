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
        admin: ResourceAddress,
        dapp_global: GlobalAddress, // The dapp_definition, not required if modifying this blueprint
    }

    impl Soulapi {
        pub fn instantiate_soul_api(
            admin: ResourceAddress,
            dapp_def: ComponentAddress,
        ) -> Global<Soulapi> {
            let dapp_global = GlobalAddress::new_or_panic(dapp_def.into());

            Self {
                soulstore_addresses: KeyValueStore::new(),
                soulstores_created: 0,
                admin,
                dapp_global,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::Fixed(rule!(require(admin))))
            .metadata(metadata! (
                roles {
                    metadata_setter => rule!(require(admin));
                    metadata_setter_updater => rule!(require(admin));
                    metadata_locker => rule!(require(admin));
                    metadata_locker_updater => rule!(require(admin));
                },
                init {
                    "name" => "soulstore creator".to_owned(), locked;
                    "description" => "Creates and tracks soulstores".to_owned(), locked;
                    "dapp_definition" => dapp_global, updatable;
                    "icon_url" => Url::of("https://soulstore.app/dark_ghost.png"), updatable;
                }
            ))
            .globalize()
        }

        pub fn create_soulstore(&mut self, global_id: NonFungibleGlobalId) {
            let soulstore_exists = self.soulstore_addresses.get(&global_id).is_some();

            assert!(!soulstore_exists, "Soulstore already exists for this NFT");

            let address =
                Soulstore::instantiate_soulstore(global_id.clone(), self.admin, self.dapp_global);

            self.soulstore_addresses.insert(global_id, address);

            self.soulstores_created += 1;
        }
    }
}
