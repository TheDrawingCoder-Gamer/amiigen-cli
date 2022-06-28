extern crate amiigen;

use amiitool_rs;
use clap::{Parser, Args};
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
enum AppArgs {
    Encrypt(InputOutputArgs),
    Decrypt(InputOutputArgs),
    Generate(GenerateArgs),
    GenerateRaw(GenerateArgsNoKey)
}
#[derive(Args, Clone, Debug)]
struct InputOutputArgs {
    #[clap(short, value_parser)]
    key_file: String,
    #[clap(short, value_parser)]
    input_file: Option<String>,
    #[clap(short, value_parser)]
    output_file: Option<String>,

}
#[derive(Args, Clone, Debug)]
struct GenerateArgs {
    #[clap(short, value_parser)]
    key_file: String, 
    #[clap(flatten)]
    da_rest : GenerateArgsNoKey

}
#[derive(Args,Clone, Debug)]
struct GenerateArgsNoKey {
    #[clap(short, long, value_parser)]
    uid: String,
    #[clap(short, long, value_parser)]
    id : String,
    #[clap(short, value_parser)]
    output_file: Option<String>
}
use std::io::{Read,Write};
fn main() -> std::io::Result<()> {
    let args = AppArgs::parse();
    match args {
        AppArgs::Encrypt(InputOutputArgs {key_file, input_file, output_file}) => {
           let data = 
               if let Some(file) = input_file {
                 std::fs::read(file)
               } else {
                 let mut stdin = std::io::stdin();
                 let mut data = Vec::new();
                 stdin.read_to_end(&mut data)?;
                 Ok(data)
               }?;
           if data.len() != 540 {
            println!("not a valid amiibo file");
            return Ok(());
           }
           let key = amiitool_rs::load_keys(&key_file).expect("invalid key");
           let res = amiitool_rs::amiibo_pack(&key, data.try_into().expect("already checked length"))?;
           if let Some(file) = output_file {
             std::fs::write(file, res)
           } else {
             let mut stdout = std::io::stdout();
             stdout.write(&res)?;
             Ok(())
           }
        }
        AppArgs::Decrypt(InputOutputArgs { key_file, input_file, output_file}) => {
           let data = 
               if let Some(file) = input_file {
                 std::fs::read(file)
               } else {
                 let mut stdin = std::io::stdin();
                 let mut data = Vec::new();
                 stdin.read_to_end(&mut data)?;
                 Ok(data)
                }?; 
           if data.len() != 540 {
            println!("not a valid amiibo file");
            return Ok(());
           }
           let key = amiitool_rs::load_keys(&key_file)?;
           let res = amiitool_rs::amiibo_unpack(&key, data.try_into().expect("already checked length"))?.get_checked()?;
           if let Some(file) = output_file {
             std::fs::write(file, res)
           } else {
             let mut stdout = std::io::stdout();

             stdout.write(&res)?;
             Ok(())
           }
                
        }
        AppArgs::Generate(GenerateArgs { key_file, da_rest: GenerateArgsNoKey {uid, id, output_file} }) => {
            let good_id : [u8; 8] = decode_hex(id.as_str()).expect("please enter valid id").try_into().expect("please enter valid id");
            let good_uid = decode_hex(uid.as_str()).expect("please enter valid tag uid");
            let amiibo = amiigen::gen_amiibo(good_id, &good_uid)?;

            let key = amiitool_rs::load_keys(&key_file)?;
            let res = amiitool_rs::amiibo_pack(&key, amiibo)?;
            if let Some(file) = output_file {
                std::fs::write(file, res)
            } else {
                let mut stdout = std::io::stdout();
                stdout.write(&res)?;
                Ok(())
            }
        }
        AppArgs::GenerateRaw(GenerateArgsNoKey {uid, id, output_file }) => {
            let good_id : [u8; 8] = decode_hex(id.as_str())?.try_into().map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "invalid id"))?;
            let good_uid = decode_hex(uid.as_str())?;
            let amiibo = amiigen::gen_amiibo(good_id, &good_uid)?;
            if let Some(file) = output_file {
                std::fs::write(file, amiibo)
           } else {
             let mut stdout = std::io::stdout();
             stdout.write(&amiibo)?;
             Ok(())
           }
        }
    }
}

fn decode_hex(s: &str) -> std::io::Result<Vec<u8>> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect::<Result<Vec<u8>, std::num::ParseIntError>>()
        .map_err(|x| std::io::Error::new(std::io::ErrorKind::InvalidData, x))
}
