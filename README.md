# Nox Futura - Rust Edition

This is a Rust port of [Nox Futura for C++](https://github.com/thebracket/bgame). I don't really write any C++ anymore, and switching brain-gear back to C++ mode was painful, so I'm porting it over to Rust. The goal is to achieve feature-parity and then expand from there.

This is very much a "free time" passion project. So there are no anticipated release dates!

## What is Nox Futura?

An open-source game combining elements of Dwarf Fortress, Civilization, Warhammer, Douglas Adams, and more. It's very much in development, so don't expect miracles - or a finished experience - yet! In particular, I've always found the building/machines parts of Dwarf Fortress fascinating, so this project focuses on that aspect. I intend to add more as I work on it.

The back-story is similar to one from the *Hitchhiker's Guide* series: a civilization (Eden) built arks to colonize distant planets. One ark was filled with the best and brightest. One with criminals - and one with the guys nobody really knows what to do with. Hairdressers, telephone sanitizers, insurance adjusters. Their ark left arly (and the rest mysteriously never took off), encountered technical difficulties is stuck in orbit above a planet. Cordex - the shipborne AI - managed to get itself and some settlers into an escape vehicle. This is the story of Cordex trying to keep the settlers alive. (I may add alternate starts at some point!)

## Tech stack

Under the hood, this project uses WGPU for rendering and Legion for ECS. There's probably too many `lazy_statics`, but they sure are convenient for shared state!

## Contributions

I'm not really looking for contributors at this point. The source code is released because I like giving back to the community, and if there's something there that helps you do something awesome - that's amazing. :-)

It's a little early for bug reports, too. There's still a *lot* that is likely to change.

## Licensing

I went with GPL because I'm all in favor of people learning from and using bits of the code, but I'd like to retain some control over the overarching game and not wake up one day to find out that someone has reskinned and released it. I'm open to changing this if people ask me enough.
