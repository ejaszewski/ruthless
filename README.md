Ruthless
--------

## About
Ruthless is an Othello AI written in Rust. This project was written for
Caltech's CS2 Othello bot competition.

## For CS2 TAs
This repo contains the code for the **Rusty** CS2 Othello team.

In this repository, the code (sans `rusty.rs`) is part of **ruthless**, which
includes a set of command line tools and a more robust command line interface
than is required/allowed for the actual CS2 competition. Thus, a separate build
target has been added specifically for the competition.

Running `cargo run` from the root of the project will run a test suite analogous
to testminimax.

Running `cargo build` will build the full CLI version of the bot, called
**ruthless** in addition to the player, **rusty**, meant to be used with the
testgame. By default, this build will be 64-bit, but other targets can be
specified using the `--target` parameter in cargo.

`testgame.cpp` and `OthelloFramework.jar` can both be found in the `util/play`.
