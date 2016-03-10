# Shouji
---
cli interface built in [rust](http://rustlang.org/) to interact with [consul](http://consul.io). Functionality was based off of [consulate](https://github.com/gmr/consulate).

#### disclaimer: this was a coding exercise to learn rust

**note: built on rust nightly**

    $ rustc --version
    $ rustc 1.6.0-nightly (1a2eaffb6 2015-10-31)

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

**cargo:**

    $ cargo install --git https://github.com/jkordish/Shouji

**binary:**

OSX:
    $ wget -O /usr/local/bin/shouji  https://github.com/jkordish/shouji/releases/download/v0.1.1/shouji-osx
Linux:
    $ wget  -O /usr/local/bin/shouji https://github.com/jkordish/shouji/releases/download/v0.1.1/shouji-linux


**source:**

    $ git clone https://github.com/jkordish/shouji.git
    $ cd shouji
    $ cargo build --release
    $ cp target/release/shouji /usr/local/bin/
    $ ./target/release/shouji get key/blah

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
    * Can't export off of the root tree
    * Remove lib curl in favor of hyper
