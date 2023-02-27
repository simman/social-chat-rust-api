use anyhow::Result;
use std::fs;
use std::path::PathBuf;

pub fn gen_proto_code(proto_file_path: &str, out_file_path: &str) -> Result<()> {
    let dir = fs::read_dir(proto_file_path)?;
    let mut out_files: Vec<String> = Vec::new();
    for (_, f) in dir.enumerate() {
        let ff = f.unwrap();

        let f_name = ff.file_name();
        let f_name = f_name.to_str().unwrap();
        if !f_name.ends_with(".protos") {
            continue;
        }

        out_files.push(String::from(f_name));

        // println!("pub mod {};", f_name);

        // let f_os_name = ff.file_name();
        // println!("{}", f_os_name.to_str().unwrap());

        let p = PathBuf::from(proto_file_path).join(f_name);

        let f_content = fs::read_to_string(p)?;
        let package_name = f_name
            .replace(".protos", "")
            .replace("S2C_", "")
            .replace("C2S_", "");
        let f_content = f_content.replace(
            "com.qx.it.protos;",
            &format!("{};", rename_package_name(package_name).as_str()),
        );

        // let out_path = PathBuf::from(out_file_path)
        //     .join(f_name);
        // fs::write(out_path, f_content)?;
    }

    // let mut config = prost_build::Config::new();
    // config.out_dir(out_file_path);
    // config.btree_map(&["."]);
    // config.type_attribute(".", "#[derive(serde::Serialize,serde::Deserialize)]");
    // ?;

    // match config.compile_protos(&out_files, &[out_file_path]) {
    //     Ok(_) => println!("生成成功～～～～"),
    //     Err(e) => panic!("{}", e),
    // }

    Ok(())
}

pub fn gen_mod(proto_file_path: &str) -> Result<()> {
    let dir = fs::read_dir(proto_file_path)?;

    let mut f_content = String::new();

    for (_, f) in dir.enumerate() {
        let ff = f.unwrap();

        let f_name = ff.file_name();
        let f_name = f_name.to_str().unwrap();
        if !f_name.ends_with(".rs") {
            continue;
        }

        // let f_content = format!("pub mod {};\n", f_name);
        f_content.push_str(&*format!("pub mod {};\n", f_name.replace(".rs", "")));
    }

    let mod_path = PathBuf::from(proto_file_path).join("protos");
    match fs::write(mod_path, f_content) {
        Ok(_) => println!("👌"),
        Err(e) => println!("{}", e),
    }

    Ok(())
}

pub fn gen_proto(proto_file_path: &str, out_file_path: &str) -> Result<()> {
    let dir = fs::read_dir(proto_file_path)?;
    let mut out_files: Vec<String> = Vec::new();
    for (_, f) in dir.enumerate() {
        let f_name = f.unwrap().path().display().to_string();
        println!("{}", f_name);
        let f_name = f_name.as_str();
        // let f_name = String::from(f_name.ok_or(Error::default())?);
        // let ff_name = f_name.ok_or(Error::default())?;
        // let f_name = f.unwrap().file_name().to_str().ok_or(Error::default())?;
        out_files.push(String::from(f_name));

        // rewrite protos file
        // let p = PathBuf::from(proto_file_path)
        //     .join(f_name.clone())
        //     .to_str()
        //     .ok_or(Error::default());
        // let f_content = fs::read_to_string(p?)?;

        // let package_name = f_name
        //     .ok_or(Error::default())?
        //     .replace(".protos", "")
        //     .replace("S2C_", "")
        //     .replace("C2S_", "");
        // let f_content = f_content.replace(
        //     "com.qx.it.protos;",
        //     rename_package_name(package_name).as_str(),
        // );
        // fs::write(p?, f_content)?;
    }

    // let mut config = prost_build::Config::new();
    // config.out_dir(out_file_path);
    // config.btree_map(&["."]);
    // config.type_attribute(".", "#[derive(serde::Serialize,serde::Deserialize)]");
    // config.compile_protos(&out_files, &[out_file_path])?;

    Ok(())
}

fn rename_package_name(package_name: String) -> String {
    let mut new_str = String::new();
    for (i, c) in package_name.chars().enumerate() {
        let mut nc = c.clone();
        let is_uppercase = nc.is_uppercase();
        if is_uppercase {
            nc = nc.to_lowercase().collect::<Vec<_>>()[0];
        }
        new_str.push(nc);
        if is_uppercase && i != 0 {
            new_str.insert(new_str.len() - 1, '_');
        }
    }
    new_str
}
