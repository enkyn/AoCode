const INPUT_DIR: &'static str = "../../common/input";
const INPUT_KEY: &'static [u8] = include_bytes!("../../common/input/inputs.key");

fn main() {
    println!("cargo:rerun-if-changed={INPUT_DIR}");

    let mut files = std::fs::read_dir(INPUT_DIR)
        .expect("failed to read input directory");

    while let Some(Ok(file)) = files.next() {
        let file_name = file.file_name();
        let (file_name, _) = file_name.to_str().unwrap().rsplit_once('.').unwrap();
        if !file_name.starts_with("day") {
            continue;
        }

        if let Ok(mut bytes) = std::fs::read(file.path()) {
            // encrypt an input
            let (nonce, mac) = x123::new(INPUT_KEY)
                .encrypt_with_data(&mut bytes, file_name.as_bytes(), None);

            // determine output directory for encrypted input
            let name = format!("{file_name}.x123");
            let mut outdir = file.path();
            outdir.pop();
            outdir.pop();

            // prepend nonce and mac to encrypted bytes
            let mut output = nonce.to_vec();
            output.extend_from_slice(&mac);
            output.extend(bytes);

            // output the encrypted input
            let path = outdir.join("crypt").join(name);
            std::fs::write(path, &output)
                .expect("failed to write encrypted file");
            
        }
    }
}