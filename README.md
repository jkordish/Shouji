# Shouji
---
rust interface for consul

#### disclaimer: this was a coding exercise to learn rust

**note: built on rust nightly**

    $ rustc --version
    $ rustc 1.5.0-nightly (78ce46ffd 2015-09-26)

Highly recommend using [multirust](https://github.com/brson/multirust) for installing and managing rust installations.

#### Features
Supports:
  * get
  * put
  * list

### Install

from source:

    $ git clone https://github.com/jkordish/shouji.git
    $ cd Shouji
    $ cargo build --release
    $ cp target/release/shouji /usr/local/bin/
    $ ./target/release/shouji get key/blah

binary:

    $ wget -O /usr/local/bin/shouji  https://github.com/jkordish/shouji/releases/download/v0.0.2/shouji-osx 

#### Using

Sample uses:

  put:

    $ shouji put random/person Jennifer

  get:

    $ shouji get random/person
    Jennifer

  list:

    $ shouji list random
    {
        "CreateIndex": 13092,
        "ModifyIndex": 13093,
        "LockIndex": 0,
        "Key": "random/person",
        "Flags": 0,
        "Value": "Jennifer"
    }


#### TODO
  * ~~List function should always return the Value field decoded from base64~~
  * Implement:
    * Export
    * Import
