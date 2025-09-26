use anyhow::anyhow;
use gtmpl_derive::Gtmpl;
use gtmpl_moyan::{Func, FuncError, Value};

fn plus_one(args: &[Value]) -> Result<Value, FuncError> {
    if let Value::Object(ref o) = &args[0] {
        if let Some(Value::Number(ref n)) = o.get("num") {
            if let Some(i) = n.as_i64() {
                return Ok((i + 1).into());
            }
        }
    }
    Err(anyhow!("integer required, got: {:?}", args).into())
}

#[derive(Gtmpl)]
struct AddMe {
    num: u8,
    plus_one: Func,
}

#[test]
fn simple_niladic_method() {
    let add_me = AddMe { num: 42, plus_one };
    let output = gtmpl_moyan::template("The answer is: {{ .plus_one }}", add_me);
    assert_eq!(&output.unwrap(), "The answer is: 43");
}
