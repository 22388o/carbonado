use serde::{Deserialize, Serialize};

/// Information from the encoding step, some of which is needed for decoding.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EncodeInfo {
    /// How many bytes input into the encoding step.
    pub input_len: u32,
    /// How many bytes total were encoded by any applicable steps for the supplied Carbonado level.
    pub output_len: u32,
    /// How large the data is after Snappy compression.
    pub bytes_compressed: u32,
    /// Compression factor.
    ///
    /// Values below 1.0 are desirable; 0.2 is typical of contracts, and 0.8 is typical of code.
    ///
    /// A value above 1.0 indicates the file grew in size, which occurs when used on incompressible file formats.
    pub compression_factor: f32,
    /// How large the data was after Ecies secp256k1 and AES-GCM authenticated encryption.
    ///
    /// This is not expected to add much overhead, typically a hundred bytes.
    pub bytes_encrypted: u32,
    /// How large the data is after adding Zfec error correction codes.
    pub bytes_ecc: u32,
    /// How large the data is after Bao encoding, for remote slice verification and integrity-checking.
    pub bytes_verifiable: u32,
    /// The total amount of file amplification. 2.0x is typical for 4/8 Zfec encoding, the others are pretty minimal, at roughly 1.1x.
    pub amplification_factor: f32,
    /// The amount of padding added to input data in order to align it with Bao slice size (1KB) and 4/8 Zfec chunk size (4KB).
    pub padding_len: u32,
    /// How many bytes are in each Zfec chunk.
    pub chunk_len: u32,
    /// How many slices are there, total.
    pub verifiable_slice_count: u16,
    /// How many slices there are per chunk.
    pub chunk_slice_count: u16,
}

/// Tuple of verifiable bytes, bao hash, and encode info struct
/// i.e., Encoded(encoded_bytes, bao_hash, encode_info)
pub struct Encoded(pub Vec<u8>, pub bao::Hash, pub EncodeInfo);
