#[cfg(target_family = "windows")]
fn main() {
  let mut res = winres::WindowsResource::new();
  res.set_manifest_file("igame_registry.exe.manifest");
  res.compile().unwrap();
}

#[cfg(not(target_family = "windows"))]
fn main() {}
