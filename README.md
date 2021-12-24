# Solana Minter

My program used to mint [Amoebits](https://moonrank.app/collection/amoebits) & [Amoebit Minis](https://moonrank.app/collection/amoebitminis). I wrote it from scratch using the hello-world program as an example & base.

# Features

* Cheap deployment costs (~2.5 solana)
* Linear mint indexes. I looked into how metaplex / candy machine works & it's not random at all & has lead to the mass botting we've seen today.
* Web3 enforced WL tokens. WL tokens can double as discount tokens.
* Calculate gas fees out of price, so the mint price is exactly correct.
* No master edition info created - no more wasted gas storing nothing. Projects launch every day that waste 20+ solana in gas a piece
* Metadata control - you can easily make changes later if you need.

# Caveats

* Meatadata URI may need to be updated post-mint to be hosted elsewhere, like Arweave or any 3rd party host. Still cheaper than storing all the config on chain.
