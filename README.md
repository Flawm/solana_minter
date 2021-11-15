# Solana Minter

My program used to mint [Amoebits](https://moonrank.app/collection/amoebits) & [Amoebit Minis](https://moonrank.app/collection/amoebitminis). I wrote it from scratch using the hello-world program as an example & base.

# Features

* Cheap deployment costs (~2.5 solana)
* Linear mint indexes. I looked into how metaplex / candy machine [works](https://github.com/metaplex-foundation/metaplex/blob/master/rust/nft-candy-machine/src/lib.rs#L572) it's not random at all, so what's the point.
* Web3 enforced WL tokens. WL tokens can double as discount tokens.
* Calculate gas fees out of price, so the mint price is exactly correct.
* No master edition info created - no more wasted gas storing nothing. Projects launch every day that waste 20+ solana in gas a piece
* Metadata control - you can easily make changes later if you need.

# Caveats

* Meatadata URI may need to be updated post-mint to be hosted elsewhere, like Arweave or any 3rd party host. Still cheaper than storing all the config on chain.
* At the time of development, I noticed the token-metadata crate had a bug in it. I had to git-clone and add it as a local dependency to pull the fix from github. Ive noticed now when I git pull things stop working - I haven't spent any time looking into this.
