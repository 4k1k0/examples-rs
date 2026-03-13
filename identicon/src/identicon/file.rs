use string_builder::Builder;

pub fn generate_filename(hash: [u8; 16]) -> String {
    let mut builder = Builder::default();

    builder.append("/tmp/");

    for h in hash{
        let tmp = format!("{}", h);
        builder.append(tmp)
    }

    builder.append(".jpg");

    builder.string().unwrap()
}
