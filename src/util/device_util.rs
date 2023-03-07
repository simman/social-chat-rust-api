use std::fs;
use std::path::PathBuf;
use uuid::{Uuid};

#[cfg(target_os = "macos")]
pub static DEFAULT_PLATFORM: &str = "macos";
#[cfg(target_os = "linux")]
pub static DEFAULT_PLATFORM: &str = "linux";
#[cfg(target_os = "windows")]
pub static DEFAULT_PLATFORM: &str = "windows";

/// 获取机器唯一识别码
pub(crate) fn serial() -> String {
    let device_id = machine_uid::get();
    match device_id {
        Ok(id) => id,
        Err(_) => {
            let user_home_dir = dirs::home_dir().unwrap();
            let machine_id_file = user_home_dir.join(".machineId");
            if std::path::Path::new(&machine_id_file).exists() {
                let id = fs::read_to_string(&machine_id_file).unwrap();
                let id = Uuid::parse_str(id.as_str());
                if id.is_ok() {
                    return id.unwrap().to_string();
                }
            }
            gen_and_write_machine_id(machine_id_file)
        }
    }
}

fn gen_and_write_machine_id(path: PathBuf) -> String {
    let id = Uuid::new_v4();
    fs::write(path, id.to_string()).unwrap();
    id.to_string()
}

#[cfg(test)]
mod test {
    use crate::util::device_util::serial;

    #[test]
    fn test_uuid() {
        let id = serial();
        println!("machineId: {}", id);
    }
}
