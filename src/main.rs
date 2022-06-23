mod data;
mod utils;

use anyhow::Context;
use std::{fs, io, process};

use data::Data;
use utils::{replace_current_dir, set_reg};

fn main() {
  if let Err(error) = try_main() {
    eprintln!("{:#?}", error);
    io::stdin().read_line(&mut String::new()).unwrap();
    process::exit(1);
  };

  println!("所有注册表全部添加成功！按回车键退出...");
  io::stdin().read_line(&mut String::new()).unwrap();
  process::exit(0);
}

fn try_main() -> anyhow::Result<()> {
  let file_string =
    fs::read_to_string("registry.yaml").with_context(|| format!("读取registry.yaml文件失败"))?;

  let data: Data = serde_yaml::from_str(&file_string)?;
  let regs = data.registry;

  for reg in regs {
    let value = replace_current_dir(reg.value.as_str())?;
    set_reg(reg.key.as_str(), value.as_str(), reg.r#type.as_str())?;
    println!("成功添加注册表{}为{}", reg.key, value);
  }

  return Ok(());
}
