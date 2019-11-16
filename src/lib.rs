/*!
# genp
This crate provides functions for generating passwords from a character list or passphrases from a word list (diceware).
Some character and word lists are exported by this crate to use when you don't want to supply your own.
## Password
```rust
use genp::password;
use genp::charlist::{LOWERCASE_CHARS, NUMBERS, UPPERCASE_CHARS};

let len = 26;

let mut charlist = Vec::new();
charlist.extend_from_slice(&LOWERCASE_CHARS);
charlist.extend_from_slice(&NUMBERS);
charlist.extend_from_slice(&UPPERCASE_CHARS);

println!("{}", password(&charlist, len));
```

## Passphrase
```rust
use genp::diceware;
use genp::wordlist::ENGLISH;

let len = 12;

println!("{}", diceware(&ENGLISH, len));
```
*/

use rand::Rng;

fn choose_from<T: Copy>(arr: &[T]) -> T {
    let index = rand::thread_rng().gen_range(0, arr.len());
    arr[index]
}

/// Generate a password of length `len` from a character list.
pub fn password(charlist: &[char], len: u8) -> String {
    let mut password = String::new();
    for _ in 0..len {
        let character = choose_from(charlist);
        password.push(character);
    }
    password
}

/// Generate a passphrase of length `len` from a wordlist.
pub fn diceware(wordlist: &[&str], len: u8) -> String {
    let mut password = Vec::new();
    for _ in 0..len {
        password.push(choose_from(wordlist));
    }
    password.join(" ")
}

pub mod charlist;
pub mod wordlist;
