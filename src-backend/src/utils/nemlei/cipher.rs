use crate::utils::nemlei::versions;
use crate::utils::nemlei::nfe;
use crate::utils::nemlei::k9a;

// decrypt data, wrapper for different game versions
pub fn decrypt(data: &[u8], file_path: &str) -> Vec<u8> {
    match versions::get_functional_version() {
        versions::MajorGameVersions::V3_0_3 => nfe::decrypt(data, file_path),
        versions::MajorGameVersions::V2_0_14 => k9a::decrypt(data, file_path),
    }
}

// decrypt file, wrapper for different game versions (with extension)
pub fn decrypt_file(file_path: &str) -> (Vec<u8>, String) {
    match versions::get_functional_version() {
        versions::MajorGameVersions::V3_0_3 => nfe::decrypt_file(file_path),
        versions::MajorGameVersions::V2_0_14 => k9a::decrypt_file(file_path),
    }
}

// encrypt data, wrapper for different game versions
#[allow(dead_code)]
pub fn encrypt(data: &[u8], file_path: &str, advanced_positions: bool) -> Vec<u8> {
    match versions::get_functional_version() {
        versions::MajorGameVersions::V3_0_3 => nfe::encrypt(data, file_path, advanced_positions),
        versions::MajorGameVersions::V2_0_14 => k9a::encrypt(data, file_path, advanced_positions),
    }
}

// encrypt file, wrapper for different game versions
pub fn encrypt_file(file_path: &str, advanced_positions: bool) -> (Vec<u8>, String) {
    match versions::get_functional_version() {
        versions::MajorGameVersions::V3_0_3 => nfe::encrypt_file(file_path, advanced_positions),
        versions::MajorGameVersions::V2_0_14 => k9a::encrypt_file(file_path, advanced_positions),
    }
}