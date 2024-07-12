use std::error::Error;
use std::fmt;
use std::mem;

use yaml_rust as yaml;

use crate::map::Map;
use crate::value::{Value, ValueKind};

pub fn parse(
    uri: Option<&String>,
    text: &str,
) -> Result<Map<String, Value>, Box<dyn Error + Send + Sync>> {
    // Parse a YAML object from file
    let mut docs = yaml::YamlLoader::load_from_str(text)?;
    let root = match docs.len() {
        0 => yaml::Yaml::Hash(yaml::yaml::Hash::new()),
        1 => mem::replace(&mut docs[0], yaml::Yaml::Null),
        n => {
            return Err(Box::new(MultipleDocumentsError(n)));
        }
    };

    // TODO: Have a proper error fire if the root of a file is ever not a Table
    let value = from_yaml_value(uri, &root)?;
    match value.kind {
        ValueKind::Table(map) => Ok(map),

        _ => Ok(Map::new()),
    }
}

fn from_yaml_value(
    uri: Option<&String>,
    value: &yaml::Yaml,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    match *value {
        yaml::Yaml::String(ref value) => Ok(Value::new(uri, ValueKind::String(value.clone()))),
        yaml::Yaml::Real(ref value) => {
            // TODO: Figure out in what cases this can panic?
            value
                .parse::<f64>()
                .map_err(|_| {
                    Box::new(FloatParsingError(value.to_string())) as Box<(dyn Error + Send + Sync)>
                })
                .map(ValueKind::Float)
                .map(|f| Value::new(uri, f))
        }
        yaml::Yaml::Integer(value) => Ok(Value::new(uri, ValueKind::I64(value))),
        yaml::Yaml::Boolean(value) => Ok(Value::new(uri, ValueKind::Boolean(value))),
        yaml::Yaml::Hash(ref table) => {
            let mut m = Map::new();
            for (key, value) in table {
                if let Some(k) = key.as_str() {
                    m.insert(k.to_owned(), from_yaml_value(uri, value)?);
                }
                // TODO: should we do anything for non-string keys?
            }
            Ok(Value::new(uri, ValueKind::Table(m)))
        }
        yaml::Yaml::Array(ref array) => {
            let mut l = Vec::new();

            for value in array {
                l.push(from_yaml_value(uri, value)?);
            }

            Ok(Value::new(uri, ValueKind::Array(l)))
        }

        // 1. Yaml NULL
        // 2. BadValue – It shouldn't be possible to hit BadValue as this only happens when
        //               using the index trait badly or on a type error but we send back nil.
        // 3. Alias – No idea what to do with this and there is a note in the lib that its
        //            not fully supported yet anyway
        _ => Ok(Value::new(uri, ValueKind::Nil)),
    }
}

#[derive(Debug, Copy, Clone)]
struct MultipleDocumentsError(usize);

impl fmt::Display for MultipleDocumentsError {
    fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
        write!(format, "Got {} YAML documents, expected 1", self.0)
    }
}

impl Error for MultipleDocumentsError {
    fn description(&self) -> &str {
        "More than one YAML document provided"
    }
}

#[derive(Debug, Clone)]
struct FloatParsingError(String);

impl fmt::Display for FloatParsingError {
    fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
        write!(format, "Parsing {} as floating point number failed", self.0)
    }
}

impl Error for FloatParsingError {
    fn description(&self) -> &str {
        "Floating point number parsing failed"
    }
}
