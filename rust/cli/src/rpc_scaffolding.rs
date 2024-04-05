use base64::engine::general_purpose;
use base64::Engine;
use quary_core::databases::DatabaseConnection;
use std::future::Future;
use std::pin::Pin;

pub fn rpc_wrapper<Fut, F, Req, Res>(
    f: F,
) -> Box<
    dyn FnOnce(
        String,
        Box<dyn DatabaseConnection>,
    ) -> Pin<Box<dyn Future<Output = Result<String, String>>>>,
>
where
    F: FnOnce(Req, Box<dyn DatabaseConnection>) -> Fut + 'static,
    Req: prost::Message + Default,
    Res: prost::Message,
    Fut: Future<Output = Result<Res, String>> + 'static,
{
    Box::new(
        move |request: String, database: Box<dyn DatabaseConnection>| {
            Box::pin(async move {
                let decoded_request = decode(&request).map_err(|e| format!("error {}", e))?;
                let response = f(decoded_request, database)
                    .await
                    .map_err(|e| format!("error {}", e))?;
                let encoded_response = encode(response).map_err(|e| format!("error {}", e))?;
                Ok(encoded_response)
            })
        },
    )
}

fn decode<Req: prost::Message + Default>(req: &str) -> Result<Req, String> {
    let bytes = general_purpose::STANDARD
        .decode(req)
        .map_err(|e| e.to_string())?;

    let req = Req::decode(&bytes[..]).map_err(|e| e.to_string())?;
    Ok(req)
}

fn encode<Res: prost::Message>(res: Res) -> Result<String, String> {
    let mut buf = Vec::new();
    res.encode(&mut buf).map_err(|e| e.to_string())?;

    let encoded = general_purpose::STANDARD.encode(&buf);
    Ok(encoded)
}
