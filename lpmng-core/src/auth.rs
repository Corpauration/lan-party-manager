use biscuit_auth::{Biscuit, KeyPair, PrivateKey};
use sha256::digest;

pub fn build_token(role: String, id: String, private_key: PrivateKey) -> Option<String> {
    let root = KeyPair::from(&private_key);

    let mut builder = Biscuit::builder();

    builder
        .add_fact(format!("role(\"{}\")", role).as_str())
        .ok()?;

    builder
        .add_fact(format!("id(\"{}\")", id).as_str())
        .ok()?;

    let biscuit = builder.build(&root).ok()?;

    biscuit.to_base64().ok()
}

pub fn check_admin(auth_token: String, private_key: PrivateKey) -> bool {
    match Biscuit::from_base64(auth_token, |_| Ok(private_key.public())) {
        Ok(t) => {
            let mut auth = t.authorizer().unwrap();

            let _ = auth.add_code("allow if role(\"admin\")");

            auth.authorize().is_ok()
        }
        Err(_) => false,
    }
}

pub fn check_id(id: String, auth_token: String, private_key: PrivateKey) -> bool {
    match Biscuit::from_base64(auth_token, |_| Ok(private_key.public())) {
        Ok(t) => {
            let mut auth = t.authorizer().unwrap();

            let _ = auth.add_code(format!("allow if id(\"{id}\")"));

            auth.authorize().is_ok()
        }
        Err(_) => false,
    }
}

pub fn hash(input: String) -> String {
    digest(input)
}

pub fn check_hash(input: String, h: String) -> bool {
    hash(input) == h
}
