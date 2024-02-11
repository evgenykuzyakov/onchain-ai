use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Clone)]
#[borsh(crate = "near_sdk::borsh")]
pub enum VResponse {
    Current(Response),
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[borsh(crate = "near_sdk::borsh")]
#[serde(crate = "near_sdk::serde")]
pub struct Response {
    pub ok: bool,
    pub text: Option<String>,
}

impl From<VResponse> for Response {
    fn from(v: VResponse) -> Self {
        match v {
            VResponse::Current(response) => response,
        }
    }
}

impl From<Response> for VResponse {
    fn from(response: Response) -> Self {
        VResponse::Current(response)
    }
}
