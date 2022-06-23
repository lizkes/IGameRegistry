use anyhow::anyhow;
use winreg::enums::*;
use winreg::{RegKey, RegValue};

pub fn set_reg(key: &str, value: &str, r#type: &str) -> anyhow::Result<()> {
  let vec: Vec<&str> = key.split(r"\").collect();
  let vec_length = vec.len();
  let predef = vec[0];
  let name = vec[vec_length - 1];
  let subkey = vec[1..vec_length - 1].join(r"\");

  let pre_key: RegKey;
  match predef {
    "HKLM" | "HKEY_LOCAL_MACHINE" => pre_key = RegKey::predef(HKEY_LOCAL_MACHINE),
    "HKCU" | "HKEY_CURRENT_USER" => pre_key = RegKey::predef(HKEY_CURRENT_USER),
    _ => return Err(anyhow!("不支持的predef: {}", predef)),
  }

  let (settings, _) = pre_key.create_subkey(subkey)?;
  match r#type {
    "REG_SZ" => settings.set_value(name, &value)?,
    "REG_DWORD" => settings.set_value(name, &u32::from_str_radix(value, 16)?)?,
    "REG_BINARY" => {
      let value = value.replace(",", "");
      let data = RegValue {
        vtype: REG_BINARY,
        bytes: hex::decode(value)?,
      };
      settings.set_raw_value(name, &data)?;
    }
    _ => return Err(anyhow!("不支持的type: {}", r#type)),
  }

  return Ok(());
}
