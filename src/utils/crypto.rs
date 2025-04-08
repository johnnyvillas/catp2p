/* Copyright 2025 Joao Guimaraes, Catp2p Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! Cryptographic utilities.

use crate::error::Error;
use rand::Rng;
use sha2::{Sha256, Digest};
use std::fmt;

/// A hash value.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Hash([u8; 32]);

impl Hash {
    /// Creates a new hash from the given bytes.
    pub fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
    
    /// Creates a hash from the given data.
    pub fn from_data(data: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&result);
        
        Self(bytes)
    }
    
    /// Returns the hash as bytes.
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl fmt::Debug for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hash({})", hex::encode(&self.0))
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(&self.0))
    }
}

/// Generates a random nonce.
pub fn generate_nonce() -> [u8; 16] {
    let mut rng = rand::thread_rng();
    let mut nonce = [0u8; 16];
    rng.fill(&mut nonce);
    nonce
}

/// Verifies a signature.
pub fn verify_signature(public_key: &[u8], message: &[u8], signature: &[u8]) -> Result<bool, Error> {
    #[cfg(feature = "crypto")]
    {
        use ed25519_dalek::{PublicKey, Signature, Verifier};
        
        // Parse the public key
        let public_key = PublicKey::from_bytes(public_key)
            .map_err(|e| Error::Crypto(format!("Invalid public key: {}", e)))?;
        
        // Parse the signature
        let signature = Signature::from_bytes(signature)
            .map_err(|e| Error::Crypto(format!("Invalid signature: {}", e)))?;
        
        // Verify the signature
        Ok(public_key.verify(message, &signature).is_ok())
    }
    
    #[cfg(not(feature = "crypto"))]
    {
        Err(Error::Crypto("Crypto support is not enabled".to_string()))
    }
}

/// Signs a message.
pub fn sign_message(private_key: &[u8], message: &[u8]) -> Result<Vec<u8>, Error> {
    #[cfg(feature = "crypto")]
    {
        use ed25519_dalek::{Keypair, Signer};
        
        // Parse the private key
        let keypair = Keypair::from_bytes(private_key)
            .map_err(|e| Error::Crypto(format!("Invalid private key: {}", e)))?;
        
        // Sign the message
        let signature = keypair.sign(message);
        
        Ok(signature.to_bytes().to_vec())
    }
    
    #[cfg(not(feature = "crypto"))]
    {
        Err(Error::Crypto("Crypto support is not enabled".to_string()))
    }
}

/// Generates a new key pair.
pub fn generate_keypair() -> Result<(Vec<u8>, Vec<u8>), Error> {
    #[cfg(feature = "crypto")]
    {
        use ed25519_dalek::{Keypair, SECRET_KEY_LENGTH, PUBLIC_KEY_LENGTH};
        use rand::rngs::OsRng;
        
        // Generate a new key pair
        let mut csprng = OsRng{};
        let keypair = Keypair::generate(&mut csprng);
        
        let private_key = keypair.to_bytes();
        let public_key = keypair.public.to_bytes();
        
        Ok((private_key.to_vec(), public_key.to_vec()))
    }
    
    #[cfg(not(feature = "crypto"))]
    {
        Err(Error::Crypto("Crypto support is not enabled".to_string()))
    }
}
