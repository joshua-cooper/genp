/*!
# genp
This crate provides functions for generating passwords from a character list or passphrases from a word list.
Some character and word lists are exported by this crate to use when you don't want to supply your own.

## Password
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

## Passphrase
```rust
use genp::passphrase;
use genp::wordlist::ENGLISH;

let len = 12;

println!("{}", passphrase(&ENGLISH, len));
```
*/

use rand::Rng;

fn choose_from<T: Copy>(arr: &[T]) -> T {
    let index = rand::thread_rng().gen_range(0, arr.len());
    arr[index]
}

/// Generate a password of length `len` from a character list.
pub fn password(charlist: &[char], len: u128) -> String {
    let mut password = String::new();
    for _ in 0..len {
        let character = choose_from(charlist);
        password.push(character);
    }
    password
}

/// Generate a passphrase of length `len` from a wordlist.
pub fn passphrase(wordlist: &[&str], len: u128) -> String {
    let mut password = Vec::new();
    for _ in 0..len {
        password.push(choose_from(wordlist));
    }
    password.join(" ")
}

pub mod charlist;
pub mod wordlist;
