# AI completion on-chain and a bot to respond

## How to use on mainnet

- The contract is deployed at `gpt4.near`.
- The cost per request is `0.01` NEAR.
- Minimum required gas is `100` TGas.

API:

```rust
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

#[payable]
pub fn request(request: Request) -> Response {}
```

### Regular request

```bash
export NEAR_ENV=mainnet 
export CONTRACT_ID=gpt4.near
export ACCOUNT_ID=your.near.account
near call $CONTRACT_ID --accountId=$ACCOUNT_ID request '{"request": {"text": "What is the capital of the US?"}}' --amount=0.01 --gas=100000000000000
```

Potential response:
```json
{
  "ok": true,
  "text": " The capital of the United States is Washington, D.C. (District of Columbia). It's an important distinction to make, as people often confuse the capital with the country's largest city, New York.\n\nWashington, D.C. is home to many national symbols and landmarks, such as the White House, Capitol Building, and the Lincoln Memorial. The city was established in 1790 to serve as the permanent capital of the United States, and it was named after George Washington, the first U.S. President."
}
```

### JSON request

You can use JSON Schema to restrict the response into a JSON format (have to escaped into a string).
If you provide a system prompt, make sure to include word `json` (case-insensitive) to avoid an error.
```json
{
  "type": "object",
  "properties": {
    "capital": {"type": "string"}
  },
  "required": ["capital"]
}
```

```bash
export NEAR_ENV=mainnet 
export CONTRACT_ID=gpt4.near
export ACCOUNT_ID=your.near.account
near call $CONTRACT_ID --accountId=$ACCOUNT_ID request '{"request": {"text": "What is the capital of the US?", "system": "You are a helpful assistant that outputs in JSON.", "json_schema": "{\"type\": \"object\",\"properties\": {\"capital\": {\"type\": \"string\"}},\"required\": [\"capital\"]}"}}' --amount=0.01 --gas=100000000000000
```

Potential response:
```json
{
  "ok": true,
  "text": " {\n  \"capital\": \"Washington, D.C.\"\n}"
}
```

## Run your own AI assistant on NEAR blockchain

- Deploy a contract

```bash
export CONTRACT_ID=ai.testnet
export COST_MILLI_NEAR=10
nearjs deploy $CONTRACT_ID --accountId=$CONTRACT_ID contract/res/release.wasm --initFunction=new --initArgs='{"request_cost": "'$COST_MILLI_NEAR'000000000000000000000"}'
```

- Configure .env

Change the NEAR_CONTRACT_ID and NEAR_ACCOUNT_ID to your contract id, enter private key for the contract.
Note, the private key can be a limited access key, with only the method `respond` allowed.

You need to set the OPENAI_API_KEY to use the OpenAI API. Or any compatible API, e.g. from https://anyscale.com

Required .env variables:
```
NEAR_CONTRACT_ID=ai.testnet
NEAR_NETWORK_ID=testnet
NEAR_ACCOUNT_ID=ai.testnet
NEAR_PRIVATE_KEY="ed25519:"
OPENAI_API_KEY=""
```

Optional .env variables:
```
NEAR_NODE_URL="https://rpc.testnet.near.org"
OPENAI_BASE_URL="https://api.endpoints.anyscale.com/v1"
OPENAI_MODEL="mistralai/Mixtral-8x7B-Instruct-v0.1"
OPENAI_SYSTEM_PROMPT="You are a helpful assistant."
OPENAI_SYSTEM_PROMPT_JSON="You are a helpful assistant that outputs in JSON."
```

- Run the bot
```
cd respond_bot
yarn
./run.sh
```
