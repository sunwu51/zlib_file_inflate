use std::fs;
use std::str;
use std::env;
use std::process::exit;
use miniz_oxide::inflate;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        println!("please specify a file name");
        exit(1);
    }

    let file_name = &args[1][..];

    if file_name.ends_with("\\.zlib") {
        println!("please specify a file name with .zlib");
        exit(1);
    }


    let bytes : Vec<u8> = fs::read(file_name).expect("file read error");
    println!("read file finished");
    let res = inflate::decompress_to_vec_zlib(bytes.as_slice()).expect("decompress error");
    println!("decompress file finished");
    let s = match str::from_utf8(res.as_slice()) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let output_file_name =file_name.replace(".zlib", "");
    fs::write(&output_file_name, s).expect("write output error");
    println!("output_file : {}", &output_file_name);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generate_xx_zlib() {
        let bytes = miniz_oxide::deflate::compress_to_vec_zlib("hello world!".as_bytes(), 0);
        fs::write("xx.zlib", bytes);
    }
}