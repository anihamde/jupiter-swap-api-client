use {
    base64::{
        engine::general_purpose::STANDARD,
        Engine as _,
    },
    serde::{
        de::Error as _,
        ser::Error,
        Deserialize,
        Deserializer,
        Serializer,
    },
    solana_sdk::transaction::VersionedTransaction,
};

pub fn serialize<S>(t: &Option<VersionedTransaction>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match t {
        Some(t) => {
            let serialized =
                bincode::serialize(t).map_err(|e| S::Error::custom(e.to_string()))?;
            let base64_encoded = STANDARD.encode(serialized);
            s.serialize_str(base64_encoded.as_str())
        }
        None => s.serialize_none(),
    }
}

pub fn deserialize<'de, D>(d: D) -> Result<Option<VersionedTransaction>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Deserialize::deserialize(d)?;
    match s {
        Some(s) => {
            let base64_decoded = STANDARD
                .decode(s)
                .map_err(|e| D::Error::custom(e.to_string()))?;
            let transaction: VersionedTransaction = bincode::deserialize(&base64_decoded)
                .map_err(|e| D::Error::custom(e.to_string()))?;
            Ok(Some(transaction))
        }
        None => Ok(None),
    }
}
