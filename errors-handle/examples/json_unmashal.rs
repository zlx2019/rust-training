use error_handle::BusinessError;
use serde_json::Value;

fn main() {
    match json_unmashal() {
        Ok(v) => println!("{}", v),
        Err(e) => println!("error: {}", e.to_string()),
    }
}

fn json_unmashal() -> Result<Value, BusinessError> {
    let val: Value = serde_json::from_str(r#"{"name": "zhangsan", "age": 18"#)?;
    Ok(val)
}
