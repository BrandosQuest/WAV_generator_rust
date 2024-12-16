use std::error::Error;
use std::io;

fn main() {
    println!("val:{}:", 0x8E as char);

    /*const SIZE: u32 = 255;
    let mut file_len_at_riff: Vec<u8> = vec![0xff, 0xff, 0xff, 0xf0];
    println!("File length: {:?}", file_len_at_riff);
    println!("size: {:X?}", SIZE);
    println!("File length: {:?}", SIZE.to_le_bytes()); //convert to little endian
    file_len_at_riff=SIZE.to_le_bytes().to_vec();
    println!("File length: {:?}", file_len_at_riff);

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("{n} bytes read");
            println!("{} bytes read", input.len());
            println!("{input}");
            println!("{:?}", input.as_bytes());
        }
        Err(error) => println!("error: {error}"),
    }

    let number = read_4byte_u32();
    println!("Number read: {}", number);
*/


}
fn read_4byte_u32()-> u32 {
    println!("Enter 4byte u32 from 0 to 4294967295");
    let mut input = String::new();
    input= read(input);

    let output: u32 = input.trim().parse().expect("Input not an integer of 4 bytes");
    println!("output: {}", output);
    return output;

}
fn read(mut input: String)->String{
    match io::stdin().read_line(&mut input) {
        Ok(..) => {println!("{input}");
            println!("{:?}", input.as_bytes());
            input}
        Err(error) => { println!("error: {error}");
                                error.to_string()
        },
    }
}

