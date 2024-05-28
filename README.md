**[soulstore]**

**ELI5**
Soulstores are like an inventory for an NFT. They allow you to create logic that deposits and withdraws assets from an NFT. When the NFT is traded, all of the contents of its inventory are also traded with it. 

---

A soulstore is a proxy for an account component on Radix that is linked to a specific NFT (NonFungibleGlobalId). You can deposit any asset to a soulstore using the same methods found on a typical account component, such as "deposit", "deposit_batch" and their 'try_deposit' variants. Similarly, you can use 'withdraw' and 'withdraw_non_fungibles' methods on a soulstore - however, these methods are protected and require a proof of the linked NFT to be passed as the final argument.

All soulstores should be created via the soulstore_creator blueprint. This records a KeyValueStore entry of a NonFungibleGlobalId and a ComponentAddress. This allows for users to easily query the KeyValueStore to find out which components are linked to which NFTs. The NonFungibleGlobalId is the NFT you want to set up a soulstore for and the ComponentAddress is the globally accessible component linked to that NFT. Even if you don't own a particular NFT, you can create one for it. Once a soulstore is created for an NFT, another one cannot not be created by the soulstore_creator blueprint. 

It is not possible to deposit the NFT that is linked to the soulstore into the same soulstore. 

**Create a soulstore for an NFT**

**Find a soulstore for an NFT**
To find the soulstore component address linked to an NFT, you can query the Radix Babylon Gateway API here:

The KeyValueStore address is: 

**Deposit assets to a soulstore**

**Withdraw assets from a soulstore**





