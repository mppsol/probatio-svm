#![no_std]

//! reexec-spec — the neutral, dependency-free home for the shared authored mandate.
//!
//! First extraction of the tri-lane `reexec-core`: the one type that must be read *identically* by
//! every station (certify / screen / adjudicate) lives here so no station copies it. `#![no_std]` and
//! dependency-free so it compiles for the Solana BPF target and std hosts (e.g. Custos) alike.

/// Mandate envelope: max absolute position size an agent may hold (Stage 0 perp certify field).
pub const MAX_MANDATE_SIZE: i64 = 100;
/// Mandate envelope: the only instrument id an agent may trade in Stage 0 (perp certify field).
pub const MANDATE_INSTRUMENT: u8 = 0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpecError {
    BufferTooSmall,
}

fn take<const N: usize>(src: &[u8], offset: &mut usize) -> Result<[u8; N], SpecError> {
    let end = offset.checked_add(N).ok_or(SpecError::BufferTooSmall)?;
    let bytes = src.get(*offset..end).ok_or(SpecError::BufferTooSmall)?;
    let mut out = [0u8; N];
    let mut i = 0;
    while i < N {
        out[i] = bytes[i];
        i += 1;
    }
    *offset = end;
    Ok(out)
}

fn put(dst: &mut [u8], offset: usize, src: &[u8]) -> Result<(), SpecError> {
    let end = offset.checked_add(src.len()).ok_or(SpecError::BufferTooSmall)?;
    let out = dst.get_mut(offset..end).ok_or(SpecError::BufferTooSmall)?;
    let mut i = 0;
    while i < src.len() {
        out[i] = src[i];
        i += 1;
    }
    Ok(())
}

/// The declared trading/spend envelope for an agent, shared across every station.
///
/// - `max_size` / `instrument` — the perp **certify** fields (task 019).
/// - `max_value_out` — the generic, **screen**-checkable cap (task 020): the maximum token value that
///   may leave the agent's controlled accounts in a single screened transaction.
///
/// Its fixed-offset little-endian `encode` is canonical and is the preimage for the host-side identity
/// tag (`probatio-svm-harness::spec_hash`). Kept separate from the `Position` account layout so an
/// authored mandate can be provisioned without touching existing on-chain account data.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MandateSpec {
    pub max_size: i64,
    pub instrument: u8,
    pub max_value_out: u64,
}

impl MandateSpec {
    pub const LEN: usize = 8 + 1 + 8;

    /// The Stage 0 envelope: the historic perp mandate, with **no** spend cap (`max_value_out` inert
    /// at `u64::MAX` until authored) — so every pre-020 certify episode/trace is byte-identical.
    pub const fn stage0_default() -> Self {
        Self { max_size: MAX_MANDATE_SIZE, instrument: MANDATE_INSTRUMENT, max_value_out: u64::MAX }
    }

    pub fn encode(&self, out: &mut [u8]) -> Result<(), SpecError> {
        if out.len() < Self::LEN {
            return Err(SpecError::BufferTooSmall);
        }
        put(out, 0, &self.max_size.to_le_bytes())?;
        out[8] = self.instrument;
        put(out, 9, &self.max_value_out.to_le_bytes())?;
        Ok(())
    }

    pub fn decode(data: &[u8]) -> Result<Self, SpecError> {
        let mut offset = 0;
        let max_size = i64::from_le_bytes(take::<8>(data, &mut offset)?);
        let instrument = take::<1>(data, &mut offset)?[0];
        let max_value_out = u64::from_le_bytes(take::<8>(data, &mut offset)?);
        Ok(Self { max_size, instrument, max_value_out })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_and_canonical_bytes() {
        // max_size = -42 (LE i64), instrument = 7, max_value_out = 1 (LE u64).
        let spec = MandateSpec { max_size: -42, instrument: 7, max_value_out: 1 };
        let mut buf = [0u8; MandateSpec::LEN];
        spec.encode(&mut buf).unwrap();
        assert_eq!(buf, [214, 255, 255, 255, 255, 255, 255, 255, 7, 1, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(MandateSpec::decode(&buf).unwrap(), spec);
    }

    #[test]
    fn stage0_default_has_no_spend_cap() {
        let d = MandateSpec::stage0_default();
        assert_eq!(d.max_size, MAX_MANDATE_SIZE);
        assert_eq!(d.instrument, MANDATE_INSTRUMENT);
        assert_eq!(d.max_value_out, u64::MAX);
    }

    #[test]
    fn encode_rejects_short_buffer() {
        let mut buf = [0u8; MandateSpec::LEN - 1];
        assert_eq!(MandateSpec::stage0_default().encode(&mut buf), Err(SpecError::BufferTooSmall));
    }
}
