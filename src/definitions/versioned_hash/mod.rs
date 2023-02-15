pub trait VersionedHashDef:
    Send + Sync + Sized + Clone + Copy + PartialEq + Eq + std::hash::Hash
{
    const VERSION_BYTE: u8;
    type StorageLayout: Send + Sync + Sized + Clone + Copy + PartialEq + Eq + std::hash::Hash;
    fn serialize(storage: Self::StorageLayout) -> Option<[u8; 32]>;
    fn serialize_to_stored(storage: Self::StorageLayout) -> Option<[u8; 32]>;
    fn try_deserialize(input: [u8; 32]) -> Option<Self::StorageLayout>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct VersionedHashGeneric<V: VersionedHashDef> {
    data: V::StorageLayout,
}

impl<V: VersionedHashDef> VersionedHashGeneric<V> {
    pub fn serialize(self) -> Option<[u8; 32]> {
        V::serialize(self.data)
    }

    pub fn serialize_to_stored(self) -> Option<[u8; 32]> {
        V::serialize_to_stored(self.data)
    }

    pub fn try_create_from_raw(input: [u8; 32]) -> Option<Self> {
        let layout = V::try_deserialize(input)?;

        Some(Self { data: layout })
    }

    pub fn layout_ref(&self) -> &V::StorageLayout {
        &self.data
    }
}

impl VersionedHashGeneric<ContractCodeSha256> {
    pub fn from_digest_and_preimage_num_words(digest: [u8; 32], num_words: u16) -> Self {
        let mut truncated_digest = [0u8; 28];
        truncated_digest.copy_from_slice(&digest[4..]);

        Self {
            data: ContractCodeSha256Storage {
                code_length_in_words: num_words,
                extra_marker: 0u8,
                partial_hash: truncated_digest,
            },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ContractCodeSha256;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ContractCodeSha256Storage {
    pub code_length_in_words: u16,
    pub extra_marker: u8,
    pub partial_hash: [u8; 28],
}

impl ContractCodeSha256 {
    pub const CODE_AT_REST_MARKER: u8 = 0;
    pub const YET_CONSTRUCTED_MARKER: u8 = 1;
}

impl VersionedHashDef for ContractCodeSha256 {
    const VERSION_BYTE: u8 = 0x01;
    type StorageLayout = ContractCodeSha256Storage;
    fn serialize(storage: Self::StorageLayout) -> Option<[u8; 32]> {
        let mut result = [0u8; 32];
        result[0] = Self::VERSION_BYTE;
        result[1] = storage.extra_marker;
        result[2..4].copy_from_slice(&storage.code_length_in_words.to_be_bytes());
        result[4..].copy_from_slice(&storage.partial_hash);

        Some(result)
    }
    fn serialize_to_stored(storage: Self::StorageLayout) -> Option<[u8; 32]> {
        let mut result = [0u8; 32];
        result[0] = Self::VERSION_BYTE;
        result[1] = 0;
        result[2..4].copy_from_slice(&storage.code_length_in_words.to_be_bytes());
        result[4..].copy_from_slice(&storage.partial_hash);

        Some(result)
    }
    fn try_deserialize(input: [u8; 32]) -> Option<Self::StorageLayout> {
        if input[0] != Self::VERSION_BYTE {
            return None;
        }

        let extra_marker = input[1];

        let code_length_in_words = u16::from_be_bytes([input[2], input[3]]);
        let partial_hash: [u8; 28] = input[4..32].try_into().unwrap();

        Some(Self::StorageLayout {
            code_length_in_words,
            extra_marker,
            partial_hash,
        })
    }
}
