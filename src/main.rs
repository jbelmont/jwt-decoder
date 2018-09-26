extern crate base64;

use base64::{encode, decode};
use std::env;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Algorithm {
    HS256,
    HS384,
    HS512,
    RS256,
    RS384,
    RS512,
    ES256,
    ES384,
    ES512
}

pub struct Header {
    typ: String,
    alg: Algorithm,
}

pub struct Token<T> {
    header: Header,
    claims: T,
    signature: String,
}

pub struct Claims {
    sub: String,
    iss: String,
    aud: String,
    exp: u32,
    nbf: u32,
    iat: u32,
    jti: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];

    let decode_jwt = &decode(file).unwrap()[..];
    println!("{:?}", decode_jwt);
}