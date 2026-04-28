#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set("ProductName", "llama.cpp lunch");
    res.set("FileDescription", "llama.cpp lunch - GUI Launcher");
    res.set("LegalCopyright", "Copyright 2025");
    res.set("InternalName", "llama.cpp lunch");
    res.set("OriginalFilename", "llama-cpp-lunch.exe");
    res.set_version_info(winres::VersionInfo::FILEVERSION, 0x0000000100000000u64);
    res.set_version_info(winres::VersionInfo::PRODUCTVERSION, 0x0000000100000000u64);
    res.set_version_info(winres::VersionInfo::FILEOS, 0x40004u64);
    res.set_version_info(winres::VersionInfo::FILETYPE, 1u64);
    res.compile().unwrap();
}

#[cfg(not(windows))]
fn main() {
    // No-op on non-Windows platforms
}
