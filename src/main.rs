fn main() {
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
