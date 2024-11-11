use crate::error::Error::BiscuitMalformed;
use crate::error::Result;
use biscuit_auth::{Biscuit, KeyPair, PrivateKey};
use password_auth::{generate_hash, verify_password};
use uuid::Uuid;

pub fn build_token(role: String, id: Uuid, private_key: &PrivateKey) -> Result<String> {
    let root = KeyPair::from(private_key);

    let mut builder = Biscuit::builder();

    builder.add_fact(format!("role(\"{}\")", role).as_str())?;

    builder.add_fact(format!("id(\"{}\")", id).as_str())?;

    let biscuit = builder.build(&root)?;

    biscuit.to_base64().map_err(Into::into)
}

pub fn check_admin(auth_token: String, private_key: &PrivateKey) -> Result<bool> {
    let t = Biscuit::from_base64(auth_token, |_| Ok(private_key.public()))?;

    let mut auth = t.authorizer()?;
    auth.add_code("allow if role(\"admin\")")?;

    Ok(auth.authorize().is_ok())
}

pub fn check_id(id: Uuid, auth_token: String, private_key: &PrivateKey) -> Result<bool> {
    let t = Biscuit::from_base64(auth_token, |_| Ok(private_key.public()))?;

    let mut auth = t.authorizer()?;
    auth.add_code(format!("allow if id(\"{id}\")"))?;

    Ok(auth.authorize().is_ok())
}

pub fn get_id(auth_token: String, private_key: &PrivateKey) -> Result<String> {
    let t = Biscuit::from_base64(auth_token, |_| Ok(private_key.public()))?;

    let mut auth = t.authorizer()?;
    let res: Vec<(String,)> = auth.query("data($id) <- id($id)")?;

    res.first().map(|e| e.0.clone()).ok_or(BiscuitMalformed)
}

pub fn hash(input: String) -> String {
    generate_hash(input)
}

pub fn check_hash(input: String, h: String) -> bool {
    verify_password(input, h.as_str()).is_ok()
}
