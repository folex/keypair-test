use fluence_keypair::KeyPair;

fn main() {
    println!("Hello, world! {:?}", KeyPair::generate_ed25519().public());
}
