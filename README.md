# **[soulstore]**

Soulstores are like an inventory for an NFT. They allow you to create logic that deposits and withdraws assets from an NFT. When the NFT is traded, all of the contents of its inventory are also traded with it. 

---

A soulstore is a proxy for an account component on Radix that is linked to a specific NFT (NonFungibleGlobalId). You can deposit any asset to a soulstore using the same methods found on a typical account component, such as "deposit", "deposit_batch" and their 'try_deposit' variants. Similarly, you can use 'withdraw' and 'withdraw_non_fungibles' methods on a soulstore - however, these methods are protected and require a proof of the linked NFT to be passed as the final argument.

All soulstores should be created via the soulstore_creator blueprint. This records a KeyValueStore entry of a NonFungibleGlobalId and a ComponentAddress. This allows for users to easily query the KeyValueStore to find out which components are linked to which NFTs. The NonFungibleGlobalId is the NFT you want to set up a soulstore for and the ComponentAddress is the globally accessible component linked to that NFT. Even if you don't own a particular NFT, you can create one for it. Once a soulstore is created for an NFT, another one cannot not be created by the soulstore_creator blueprint. 

It is not possible to deposit the NFT that is linked to the soulstore into the same soulstore. 

## **Create a soulstore for an NFT**

```
CALL_METHOD
Address("component_rdx1crkftuplrghgcc8ffl0lf5gr4jlx62u9d54at8q9c5lve65jzz46m7")
"create_soulstore"
NonFungibleGlobalId("${NonFungibleGlobalId}")
;
```

## **Find a soulstore for an NFT**
To find the soulstore component address linked to an NFT, you can query the Radix Babylon Gateway API here: https://radix-babylon-gateway-api.redoc.ly/#operation/KeyValueStoreData

Use the JSON Key with the soulstore_creator internal_keyvaluestore address. 

An example of query:

```
key_value_store_address: "internal_keyvaluestore_rdx1kqhs6kd8nvpz7lfqgujyjfksmvt3q7r09l3w3pz32lxh6zg9pyz69w",
keys: [
    {
      "key_json": {
        "kind": "Tuple",
        "fields": [
          {
            "value": {Insert some resource address},
            "kind": "Reference",
            "type_name": "ResourceAddress",
            "field_name": "resource_address"
          },
          {
            "value": {Insert some local id},
            "kind": "NonFungibleLocalId",
            "field_name": "local_id"
          }
        ]
      }
    }
  ]
```

## **Deposit assets to a soulstore**

You can use all the same deposit methods found on typical account component. This includes, "deposit", "deposit_batch", "try_deposit_or_abort", "try_deposite_or_refund", "try_deposit_batch_or_abort" and "try_deposit_batch_or_refund". The 'try' variations of deposits just accept None as their authoriser - you can't set a soulstore to block third-party deposits.

## **Withdraw assets from a soulstore**

Methods similar to a typical account can also be found, including "withdraw' for tokens and "withdraw_non_fungibles". However, the last argument passed in the method must be a Proof of the NFT that the soulstore is linked to.

## **Read-only methods on soulstores**

There are three read-only methods on soulstore components which can be used to check the contents of a soulstore via Scyrpto. The three methods include checking if a resource vault is present (note that a vault could be there, but empty), checking for an amount of resource > or =, checking a specific NFT is present:

```
 pub fn has_resource(&self, resource_address: ResourceAddress) -> bool
```
```
 pub fn has_atleast_amount(&self, resource_address: ResourceAddress, amount: Decimal) -> bool
```
```
 pub fn has_non_fungible_id(&self, resource_address: ResourceAddress, non_fungible_local_id: NonFungibleLocalId) -> bool
```

## **Potential Use-Cases**

1. **Transfer NFTs in bulk**: The Radix Engine currently limits the amount of events that can happen in a single transaction. This means that its only possible to transfer approx. 60 NFTs in a single transaction. By utilising a soulstore, its possible to trade 1 NFT that 'contains' an near unlimited amount. This could also be a more specific feature of the NFT, for example an RPG character NFT that has battled to earn equipment NFTs - you could trade the character NFT along with all its equipment. 

2. **Airdrops to NFT holders**: If an NFT is sitting in a component, for instance on an NFT marketplace, it can be tricky to find or decide who the current owner is. Rather than airdropping to a specific account - you can just airdrop to the NFT more directly. 

3. **NFT slots**: While this isn't specifically built into the current soulstore component - it serves as a great template for having NFTs with specific slots which certain NFTs can be attached to. Think again of an RPG character that can only have one sword NFT equipped at a time. 

4. **Makeshift account balance within Scrypto**: Sometimes it can be frustrating to not know who the caller of a method is or be able to query their account balance within Scrypto code. However, for certain purposes, soulstores could be useful as they each have read-only methods for querying a specific NFTs fungible and non-fungible contents. 



