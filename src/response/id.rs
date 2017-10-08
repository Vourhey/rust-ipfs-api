use response::serde;


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IdResponse {
    #[serde(rename = "ID")]
    id: String,

    public_key: String,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    addresses: Vec<String>,

    agent_version: String,
    protocol_version: String,
}


#[cfg(test)]
mod tests {
    deserialize_test!(v0_id_0, IdResponse);
}