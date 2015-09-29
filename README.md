# Shouji
---
rust interface for consul

#### disclaimer: this was a coding exercise to learn rust

**note: built on rust nightly**

    $ rustc --version
    $ rustc 1.5.0-nightly (6108e8c3e 2015-09-28)

Highly recommend using [multirust](https://github.com/brson/multirust) for installing and managing rust installations.

#### Features
Supports:
  * get
  * put
  * rm
  * list
  * export
  * import

### Install

**source:**

    $ git clone https://github.com/jkordish/shouji.git
    $ cd Shouji
    $ cargo build --release
    $ cp target/release/shouji /usr/local/bin/
    $ ./target/release/shouji get key/blah

**binary:**

    $ wget -O /usr/local/bin/shouji  https://github.com/jkordish/shouji/releases/download/v0.0.2/shouji-osx

#### Using

Sample uses:

  *put:*

    $ shouji put random/person Jennifer

  *get:*

    $ shouji get random/person
    Jennifer

  *list:*

    $ shouji list random
    {
        "CreateIndex": 13092,
        "ModifyIndex": 13093,
        "LockIndex": 0,
        "Key": "random/person",
        "Flags": 0,
        "Value": "Jennifer"
    }

    *export:*

      $ shouji export random file
      [
        {
          "Key": "random/person",
          "Flags": 0,
          "Value": "Jennifer"
        }
      ]

      *import:*

        $ <edit file to replace value to Joe>
        $ shouji import file
        $ shouji get random/person
        Joe

  *remove:*

    $ shouji rm random/person

#### TODO
  * Implement:
    * ~~Export~~
    * ~~Import~~
    * Can't export off of the root tree :(
