fn main() {
  let message = ["Thanks for installing Platypus!","We're trying to make the world a better place.","We're very happy you've decided to get involved."];

  for sptr in message.iter() {
    println!("{}",*sptr);
  }
}
