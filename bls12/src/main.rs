use hex;
use lambdaworks::bls12_381::{Bls12381, Scalar, G1};
fn main() {
    let secret_key_hex = "6C616D6264617370";

    // let secret_key_bytes = hex::decode(secret_key_hex).expect("Decoding failed");
    //     let secret_key = SecretKey::from_bytes(&secret_key_bytes).unwrap(); // `blst` framework

    //     let public_key = secret_key.sk_to_pk(); // create Public key

    //     let public_key_bytes = public_key.compress(); // transform Public key to bytes

    //     println!("Public Key: {:?}", public_key_bytes);

    let secret_key_bytes = hex::decode(secret_key_hex).expect("Decoding failed");
    let secret_key = Scalar::from_bytes_mod_order(secret_key_bytes);

    let generator = G1::generator();

    let public_key = generator * secret_key;

    let public_key_bytes = public_key.to_bytes();

    println!("Public Key: {:?}", public_key_bytes);
}
