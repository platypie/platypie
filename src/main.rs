/*
main.rs
Platypie's core executable
currently only requires Rust's standard API
*/
extern crate sodiumoxide;
extern crate serialize;

fn main() {
    use sodiumoxide::crypto::asymmetricbox::gen_keypair;
    use serialize::base64::{ToBase64, MIME};
    sodiumoxide::init();

    let (pk, sk) = gen_keypair();

    let message = [
      "Thanks for installing Platypie!",
      "We're trying to make the world a better place.",
      "We're very happy you've decided to get involved.",
      "Platypie is a work in progress.",
      "If you want to get involved, join us on IRC!",
      "irc.freenode.net/#platypie"
        ];

    for sptr in message.iter() {
        println!("{}",*sptr);
    }
}

#[test]
fn main_test() {
    assert!(true); // the simplest test possible, a tautology
}
