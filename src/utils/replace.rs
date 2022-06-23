use std::env;

use anyhow::anyhow;
use anyhow::Ok;

pub fn replace_current_dir(s: &str) -> anyhow::Result<String> {
  if s.contains(r"%current_dir%") {
    let current_exe = env::current_exe()?;
    let current_dir_str = current_exe
      .parent()
      .ok_or(anyhow!("无法获取当前执行文件的目录路径"))?
      .to_str()
      .ok_or(anyhow!("无法将当前执行文件的目录路径转换为字符串"))?;
    let value = s.replace(r"%current_dir%", current_dir_str);
    return Ok(value);
  }
  return Ok(s.to_string());
}
