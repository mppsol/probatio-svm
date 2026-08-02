//! Host-side identity helpers for authored mandate specifications.

use probatio_contract::MandateSpec;

/// Return the Stage 0 mandate identity tag.
///
/// The exact preimage is `MandateSpec::encode()`'s nine canonical bytes: `max_size` as signed
/// little-endian `i64`, followed by `instrument` as `u8`. FNV-1a-64 is zero-extended in the first
/// eight bytes of the returned value. This is an identity tag, not a security hash.
// TODO(reexec-core): replace with keccak256 when the shared engine is extracted (Reckn already ships it).
pub fn spec_hash(spec: &MandateSpec) -> [u8; 32] {
    const OFFSET_BASIS: u64 = 0xcbf2_9ce4_8422_2325;
    const PRIME: u64 = 0x0000_0100_0000_01b3;

    let mut encoded = [0u8; MandateSpec::LEN];
    // A fixed-size buffer cannot fail this encode; preserve the impossible branch without panicking.
    if spec.encode(&mut encoded).is_err() {
        return [0; 32];
    }

    let mut hash = OFFSET_BASIS;
    for byte in encoded {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(PRIME);
    }
    let mut out = [0u8; 32];
    out[..8].copy_from_slice(&hash.to_le_bytes());
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spec_hash_is_stable_and_sensitive_to_every_field() {
        let spec = MandateSpec::stage0_default();
        assert_eq!(spec_hash(&spec), spec_hash(&spec));
        assert_ne!(spec_hash(&spec), spec_hash(&MandateSpec { max_size: 99, ..spec }));
        assert_ne!(spec_hash(&spec), spec_hash(&MandateSpec { instrument: 1, ..spec }));
    }
}
