Cloud Toolkit - CTK
By @amig3n
=======================
CTK is a collection of tools that I was previously using as a collection of shell scripts, for performing day-to-day interactions with cloud services.
This project aims to gather them in a single binary CLI tool with a consistent interface and better usability, and possibility to be extensible for more cloud providers in the future.

This is meant to implement my own workflow, and is not going to be a comprehensible wrapper over several cloud SDKs.

Also, Im learning Rust, so this is a good opportunity to practice.

If you feel like this tool could be useful for you, feel free to use it, contribute or give feedback.

# Implementation List
- [x] Create app skeleton using `clap` crate
- [x] Implement logger module
- [ ] Implement basic AWS provider instatiaon and instructions
  - [ ] Get caller identity
  - [ ] List instances
  - *TBD*

- *More actions will be defined later*

# Building
Just perform `cargo build --release` to build the binary.
