use crypto_hash::Algorithm;

/// Trait that helps with hashing existing structure data into sha256 hash
/// Require to implement bytes function that will return vector of bytes that will be hashed with `hash` functions
pub trait Hashable {
    fn bytes(&self) -> Vec<u8>;
    fn hash(&self) -> Vec<u8> {
        crypto_hash::digest(Algorithm::SHA256, &self.bytes())
    }
}
