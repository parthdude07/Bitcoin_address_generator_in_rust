use k256::elliptic_curve::sec1::ToEncodedPoint;
use k256::{PublicKey, SecretKey};
use rand_core::OsRng; // Operating System Random Number Generator
use ripemd::{Ripemd160, Digest};
use sha2::Sha256;

fn main() {
    println!("--- Bitcoin Address Generator ---");
    
    // 1. Generate a random Private Key
    
    let secret_key = SecretKey::random(&mut OsRng);
    
    

    println!("1. Private Key (Hex): {}", hex::encode(secret_key.to_bytes()));

    // 2. Derive Public Key (k * G)
    
    let public_key = secret_key.public_key();
    
    // Serialize to Compressed format (33 bytes, starts with 0x02 or 0x03)
    let pub_key_bytes = public_key.to_encoded_point(true); 
    let pub_key_slice = pub_key_bytes.as_bytes();

    println!("2. Public Key (Compressed): {}", hex::encode(pub_key_slice));

    // 3. Perform SHA-256
    let mut hasher_sha256 = Sha256::new();
    hasher_sha256.update(pub_key_slice);
    let sha256_result = hasher_sha256.finalize();

    // 4. Perform RIPEMD-160 on the SHA-256 result
    let mut hasher_ripemd = Ripemd160::new();
    hasher_ripemd.update(sha256_result);
    let ripemd_result = hasher_ripemd.finalize();

    println!("3. Hash160 (Ripemd160): {}", hex::encode(ripemd_result));

    // 5. Add Network Byte (0x00 for Mainnet)
    let mut payload = Vec::new();
    payload.push(0x00); // Mainnet Version Byte
    payload.extend_from_slice(&ripemd_result);

    // 6. Calculate Checksum (Double SHA-256)
    let mut hasher_chk1 = Sha256::new();
    hasher_chk1.update(&payload);
    let chk1 = hasher_chk1.finalize();

    let mut hasher_chk2 = Sha256::new();
    hasher_chk2.update(chk1);
    let chk2 = hasher_chk2.finalize();

    // Take first 4 bytes
    let checksum = &chk2[0..4];
    
    // 7. Append Checksum to Payload
    payload.extend_from_slice(checksum);

    // 8. Base58 Encode
    let address = bs58::encode(payload).into_string();

    println!("4. Final Bitcoin Address: {}", address);
    println!("-------------------------------");

}