extern crate hocon_ext;

use self::hocon_ext::{Hocon, HoconLoader};
use source::Source;
use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use value::{Value, ValueKind};

pub fn parse(
    uri: Option<&String>,
    text: &str,
) -> Result<HashMap<String, Value>, Box<dyn Error + Send + Sync>> {
    // Parse a HOCON value from the provided text
    // TODO: Have a proper error fire if the root of a file is ever not a Table

    let data: Hocon = if uri.is_some() {
        // loading from file to properly handle includes
        HoconLoader::new().load_file(uri.unwrap())?.hocon()?
    } else {
        HoconLoader::new().load_str(text)?.hocon()?
    };

    let value = from_hocon_value(uri, &data)?;
    match value.kind {
        ValueKind::Table(map) => Ok(map),

        _ => Ok(HashMap::new()),
    }
}

fn from_hocon_value(
    uri: Option<&String>,
    value: &Hocon,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    match *value {
        Hocon::String(ref value) => Ok(Value::new(uri, value.to_string())),
        Hocon::Integer(value) => Ok(Value::new(uri, value)),
        Hocon::Real(value) => Ok(Value::new(uri, value)),
        Hocon::Boolean(value) => Ok(Value::new(uri, value)),
        Hocon::Array(ref vec) => {
            let mut result = Vec::new();
            for value in vec {
                result.push(from_hocon_value(uri, value)?);
            }
            Ok(Value::new(uri, ValueKind::Array(result)))
        }
        Hocon::Hash(ref hash) => {
            let mut result = HashMap::new();
            for (key, value) in hash {
                result.insert(key.clone(), from_hocon_value(uri, value)?);
            }
            Ok(Value::new(uri, ValueKind::Table(result)))
        }
        Hocon::Null => Ok(Value::new(uri, ValueKind::Nil)),
        Hocon::BadValue(ref e) => Err(Box::new(e.clone())),
    }
}
