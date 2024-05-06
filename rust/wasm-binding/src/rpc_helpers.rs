use js_sys::Uint8Array;
use prost::Message;

pub fn decode<T>(request: Uint8Array) -> Result<T, String>
where
    T: Message + Default,
{
    let length = request.length() as usize;
    let mut v = vec![0; length];
    request.copy_to(&mut v);

    let t = T::decode(v.as_slice()).map_err(|e| e.to_string())?;
    Ok(t)
}

pub fn encode<T>(t: T) -> Result<Uint8Array, String>
where
    T: Message,
{
    let mut buffer: Vec<u8> = vec![];
    t.encode(&mut buffer).map_err(|e| e.to_string())?;
    let js_array = Uint8Array::from(&buffer[..]);
    Ok(js_array)
}
