Ruthless
--------

[![Build Status](https://travis-ci.org/ejaszewski/ruthless.svg?branch=master)](https://travis-ci.org/ejaszewski/ruthless)
[![codecov](https://codecov.io/gh/ejaszewski/ruthless/branch/master/graph/badge.svg)](https://codecov.io/gh/ejaszewski/ruthless)
[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bgithub.com%2Fejaszewski%2Fruthless.svg?type=shield)](https://app.fossa.io/projects/git%2Bgithub.com%2Fejaszewski%2Fruthless?ref=badge_shield)

Ruthless is an Othello AI written in the [Rust] language.

The project started November 2017 as the final project for Caltech's CS2. The
current version of Ruthless is a complete rewrite started March 2018.

---

## Goals

Ruthless is a hobby project, mainly created to learn Rust, and experiment with
bitboards, code optimization, and tree searches. I also set up Travis CI and
Codecov as a learning exercise. That said, I also have some concrete goals for
the project:
- [ ] User-Friendly CLI
- [ ] Fast endgame solver
- [ ] Several search algorithms:
  - [x] Negamax
  - [ ] NegaScout
  - [x] Best Node Search
  - [ ] MCTS
- [ ] Several evaluation methods:
  - [x] Piece-Square tables
  - [ ] Pattern-Based
  - [ ] Advanced Stability & Mobility
- [ ] [NBoard](http://www.orbanova.com/nboard/) compatibility


## License
[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bgithub.com%2Fejaszewski%2Fruthless.svg?type=large)](https://app.fossa.io/projects/git%2Bgithub.com%2Fejaszewski%2Fruthless?ref=badge_large)

[Rust]: https://www.rust-lang.org
