use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Data {
  pub registry: Vec<Registry>,
}

#[derive(Debug, Deserialize)]
pub struct Registry {
  pub key: String,
  pub value: String,
  pub r#type: String,
}
