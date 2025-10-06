pub fn invoke(encoded_input: String) -> anyhow::Result<()> {
    if let Some((len, rest)) = encoded_input.split_once(":")
        && let Ok(len) = len.parse::<usize>()
    {
        println!("{}", serde_json::Value::String(rest[..len].to_string()));
        Ok(())
    } else {
        anyhow::bail!("Can not decode the given input")
    }
}
