use anyhow::{Context, Ok};

use crate::utils::bencode::{BENCODED_INT_REGEXP, BENCODED_STR_REGEXP};

pub fn invoke(encoded_input: String) -> anyhow::Result<()> {
    if BENCODED_STR_REGEXP.is_match(&encoded_input) {
        let decoded_string =
            decode_string(encoded_input).context("Can not decode the given string")?;
        println!("{}", serde_json::Value::String(decoded_string));
        Ok(())
    } else if BENCODED_INT_REGEXP.is_match(&encoded_input) {
        let decoded_int = decode_int(encoded_input).context("Can not decoded the given int")?;
        println!("{}", decoded_int);
        Ok(())
    } else {
        anyhow::bail!("Can not decode the given input")
    }
}

fn decode_string(encoded_string: String) -> anyhow::Result<String> {
    let (len, value) = BENCODED_STR_REGEXP
        .captures_iter(&encoded_string)
        .map(|c| {
            let len = c.name("len").unwrap().as_str();
            let value = c.name("value").unwrap().as_str();

            (len, value)
        })
        .next()
        .ok_or(anyhow::anyhow!("Unable to decode the given string"))?;

    let len = len
        .parse::<usize>()
        .context("Unable to parse encoded string length")?;

    Ok(value[..len].to_string())
}

fn decode_int(encoded_int: String) -> anyhow::Result<i32> {
    let value = BENCODED_INT_REGEXP
        .captures_iter(&encoded_int)
        .map(|c| c.name("value").unwrap().as_str())
        .next()
        .ok_or(anyhow::anyhow!("Unable to decode the given int"))?;

    Ok(value.parse::<i32>().context("The given int is malformed")?)
}
