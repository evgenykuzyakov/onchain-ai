use crate::*;

pub type RequestId = u32;

#[derive(BorshSerialize, BorshDeserialize, Clone)]
#[borsh(crate = "near_sdk::borsh")]
pub enum VRequest {
    Current(Request),
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[borsh(crate = "near_sdk::borsh")]
#[serde(crate = "near_sdk::serde")]
pub struct Request {
    /// The user prompt for the completion.
    pub text: String,
    /// An optional system prompt for the completion.
    pub system: Option<String>,
    /// An optional temperature for the completion.
    pub temperature: Option<f32>,
    /// An optional json schema to format the completion.
    pub json_schema: Option<String>,
}

impl From<VRequest> for Request {
    fn from(v: VRequest) -> Self {
        match v {
            VRequest::Current(request) => request,
        }
    }
}

impl From<Request> for VRequest {
    fn from(request: Request) -> Self {
        VRequest::Current(request)
    }
}
