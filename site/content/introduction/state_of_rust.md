+++
title = "The state of Rust"
weight = 30
+++

## Release channels: stable vs nightly

https://doc.rust-lang.org/book/first-edition/release-channels.html#choosing-a-version

There are two main streams to the Rust world. The first is the stable stream, and the second is the
nightly stream. The stable stream consists of a compiler and toolset that is considered a "public
release", i.e. tested and supported. Nightly is literally a nightly build of the latest development
code with all the latest whizz-bang features, not all of which are complete or fully thought out,
and may be subject to change before they are stabilized. It's sort of a test-bed for the Rust
developers to get new features out to the public and get them tested.

Unfortunately, this leads to some bifurcation in the Rust community, as while most developers are
using stable, there are quite a few interesting crates and projects that require nightly.
Furthermore, the Rust package repository, crates.io, does not distinguish between crates that
require nightly to compile and those that work on stable. It can be annoying to discover that an
interesting looking crate that you were thinking of using won't work in your project because it is
nightly-only. Unfortunately, this also applies to many of the handy tools being developed in the
Rust community such as `rustfmt`.

The Rust developers are aware of this problem, and there are many interesting and important features
due to be stabilized in 2018, whether this will move more people over to stable remains to be seen.

For now, I recommend you install stable to start with, and dabble with nightly if you need to. The
stable stream will compile all samples used in this website. It is possible to install both stable
and nightly and easily swap between them using `rustup`.

## OS Independence

Rust works well on both Windows and Linux. Since this website is targeted at C# programmers, I have
tested and run all code samples on Windows. Installation of Rust on Windows is very easy, and is
covered in the Installation section of this website.
