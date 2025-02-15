use derive_more::From;
use mlua::{ExternalError, Lua, MultiValue, Value};
use std::path::PathBuf;

pub mod manager;
mod runner;

pub enum LuaMessage {
    LoadScript(PathBuf),
    CallFunction(PathBuf, String, Vec<SerializableLuaValue>),
    Shutdown,
}

pub enum LuaResponses {
    ScriptLoaded(PathBuf),
    FunctionResult(SerializableLuaValue),
    Error(String),
}

#[derive(Debug, Clone, From)]
pub enum SerializableLuaValue {
    Nil,
    Boolean(bool),
    Integer(i64),
    Number(f64),
    String(String),
    Table(Vec<(SerializableLuaValue, SerializableLuaValue)>),
    Array(Vec<SerializableLuaValue>),
}

impl TryFrom<Value> for SerializableLuaValue {
    type Error = mlua::Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        Ok(match value {
            Value::Nil => Self::Nil,
            Value::Boolean(val) => Self::Boolean(val),
            Value::Integer(val) => Self::Integer(val),
            Value::Number(val) => Self::Number(val),
            Value::String(val) => Self::String(val.to_string_lossy()),
            Value::Table(table) => {
                // First check if it's sequence-like
                let len = table.raw_len();
                if len > 0 {
                    // Try to treat as array first
                    let mut array = Vec::with_capacity(len);
                    let mut is_sequence = true;

                    for i in 1..=len {
                        if let Ok(value) = table.raw_get::<Value>(i) {
                            array.push(SerializableLuaValue::try_from(value)?);
                        } else {
                            is_sequence = false;
                            break;
                        }
                    }

                    if is_sequence {
                        return Ok(Self::Array(array));
                    }
                }

                // If not a sequence or empty, treat as regular table
                let mut pairs = Vec::new();
                for pair in table.pairs::<Value, Value>() {
                    let (k, v) = pair?;
                    pairs.push((
                        SerializableLuaValue::try_from(k)?,
                        SerializableLuaValue::try_from(v)?,
                    ));
                }
                Self::Table(pairs)
            }
            Value::Function(_) => {
                return Err(mlua::Error::RuntimeError(
                    "Cannot serialize function".to_string(),
                ))
            }
            Value::Thread(_) => {
                return Err(mlua::Error::RuntimeError(
                    "Cannot serialize thread".to_string(),
                ))
            }
            Value::UserData(_) => {
                return Err(mlua::Error::RuntimeError(
                    "Cannot serialize userdata".to_string(),
                ))
            }
            Value::Error(e) => return Err(e.into_lua_err()),
            _ => {
                return Err(mlua::Error::RuntimeError(
                    "Unsupported value type".to_string(),
                ))
            }
        })
    }
}

impl SerializableLuaValue {
    pub fn to_lua_value(&self, lua: &Lua) -> mlua::Result<Value> {
        Ok(match self {
            SerializableLuaValue::Nil => Value::Nil,
            SerializableLuaValue::Boolean(b) => Value::Boolean(*b),
            SerializableLuaValue::Integer(i) => Value::Integer(*i),
            SerializableLuaValue::Number(n) => Value::Number(*n),
            SerializableLuaValue::String(s) => Value::String(lua.create_string(s)?),
            SerializableLuaValue::Array(arr) => {
                let table = lua.create_table()?;
                for (i, value) in arr.iter().enumerate() {
                    table.raw_set(i + 1, value.to_lua_value(lua)?)?;
                }
                Value::Table(table)
            }
            SerializableLuaValue::Table(pairs) => {
                let table = lua.create_table()?;
                for (k, v) in pairs {
                    table.raw_set(k.to_lua_value(lua)?, v.to_lua_value(lua)?)?;
                }
                Value::Table(table)
            }
        })
    }
} /*

  // Helper for converting MultiValue to/from Vec<SerializableLuaValue>
  impl TryFrom<MultiValue> for Vec<SerializableLuaValue> {
      type Error = mlua::Error;

      fn try_from(values: MultiValue) -> Result<Self, Self::Error> {
          values.into_vec().into_iter()
              .map(SerializableLuaValue::try_from)
              .collect()
      }
  }*/

impl SerializableLuaValue {
    pub fn into_args(values: Vec<Self>, lua: &Lua) -> mlua::Result<MultiValue> {
        let mut multi = MultiValue::new();
        for value in values {
            multi.push_front(value.to_lua_value(lua)?);
        }
        Ok(multi)
    }
}

#[derive(Debug, From)]
struct ScriptContext {
    environment: mlua::Table,
}
