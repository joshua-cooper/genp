# genp

This crate provides functions for generating passwords from a character list or passphrases from a word list.
Some character and word lists are exported by this crate to use when you don't want to supply your own.

## Examples

### Password

```rust
use genp::password;
use genp::charlist::{LOWERCASE, NUMBERS, UPPERCASE};

let len = 26;

let mut charlist = Vec::new();
charlist.extend_from_slice(&LOWERCASE);
charlist.extend_from_slice(&NUMBERS);
charlist.extend_from_slice(&UPPERCASE);

println!("{}", password(&charlist, len));
```

### Passphrase

```rust
use genp::passphrase;
use genp::wordlist::ENGLISH;

let len = 12;

println!("{}", passphrase(&ENGLISH, len));
```
