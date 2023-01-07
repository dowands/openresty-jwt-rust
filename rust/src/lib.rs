use std::{ffi::{CStr, CString}, error::Error};
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;
use libc::c_char;

#[no_mangle]
pub extern "C" fn jwt_vaild(key: *const c_char, auth: *const c_char) -> *const c_char {
    let key = unsafe { CStr::from_ptr(key).to_string_lossy() };
    let auth = unsafe { CStr::from_ptr(auth).to_string_lossy() };

    let result = jwt(key.into_owned().as_str(), auth.into_owned().as_str());
    let result = match result {
        Err(e) => format!("ERROR: {}", e),
        Ok(f) => f,
    };

    let returns = CString::new(result).unwrap();
    let p = returns.as_ptr();
    std::mem::forget(returns);
    p
}

fn jwt(key: &str, auth: &str) -> Result<String, Box<dyn Error>>{
    let key: Hmac<Sha256> = Hmac::new_from_slice(format!("{}", key).as_bytes())?;
    let claims: BTreeMap<String, serde_json::value::Value> = auth.verify_with_key(&key)?;
    Ok(serde_json::to_string(&claims)?)
}

#[test]
fn test(){
    match jwt("MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA+Ug6e+v7pDgqKCww6NAWN826ve9THWqnqdhrQUtcV3Kie4JZRPX7N1Dz5n+LOv0V+tjF8rKuXAqvjvUsbluSqlLZW7lW8CNGDyWPTSPnG6y7CbK09+oI0pgt38dyzUvE7j7c99t7rx8tNquKE99FjKcDwVwm3KSqae0QQwUWBvzzRJTEsUZqUJiRgfCGVNqI/u+DpqomfE48nlddLEfrXakxbbaq0Hb6CE5BalpJvHrbkrwjyDfwHQ1ynndEILn+tIBjZa6/dsHElu4dKbvpITeIQq+rkL5VzZhGZFCuHJCECb1oUj/Z0hu4EOdsPovitAbAml5L+B3B6s67AYu8nwIDAQAB"
    , "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.v-oibW8B9HQGBppk4P-jHDj-yqUgFYKu2A0GQ1aXtH4"){
        Ok(r) => println!("{:#?}", r),
        Err(e) => println!("{:#?}", e)
    }
}