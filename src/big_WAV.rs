use std::fs::File;
use std::io::Write;

fn main() {
    //write_text_file().expect("TODO: panic message");

    create_binary_file().expect("TODO: panic message");

    //pause_input_wait();
}
/*fn write_text_file() -> std::io::Result<()> {
    // Create or open a file
    let mut file = File::create("output.txt")?;

    // Write data to the file
    file.write_all(b"Hello, Rust!")?;

    println!("Data saved to output.txt");
    Ok(())
}*/
fn create_binary_file() -> std::io::Result<()>{
    let mut file = File::create("binary_output_big.bin")?;

    const DIM: usize =65491;
    /*                                                      which is the length of the entire file minus the 8 bytes for the "RIFF" and length (11598 - 11590 = 8 bytes), little endian
                                                            real length= 255+8b of "RIFF" and length ---> 263 bytes*/
    let riff_chunk_12b: [u8; 12] = [b'R', b'I', b'F', b'F', 0xF7, 0xff, 0, 0, b'W', b'A', b'V', b'E' ];
    /*                                                 ?? space                                                    */
    let format_chunk_24b: [u8; 24] = [b'f', b'm', b't', 0x20, 0x10, 0x0, 0x0, 0x0, 0x01, 0x00, 0x01, 0x00,
        /*                            SampleRateBinary,in Hz-     Bytes Per Second                          */
                                      0x22, 0x56, 0x00, 0x00, 0x22, 0x56, 0x00, 0x00,
        /*                            Bytes Per Sample:
                                        1=8 bit Mono,
                                        2=8 bit Stereo or
                                        16 bit Mono,
                                        4=16 bit Stereo           Bits Per Sample                         */
                                       0x01, 0x00,                   0x08, 0x00];
    /*                                                    Length Of Data To Follow small endian
                                                               263  real length= 219+44b of "RIFF" and length ---> 263 bytes*/
    let data_prefix_chunk_12b: [u8; 8] = [b'd', b'a', b't', b'a', 0xD3, 0xFF, 0x00, 0x00];
    // let data: [u8; DIM] = [0x80; DIM];
    let mut data: [u8; DIM] = [0x80; DIM];
    generate_wave(&mut data, DIM);


    let mut complete_data = Vec::new();
    complete_data.append(riff_chunk_12b.to_vec().as_mut());
    complete_data.append(format_chunk_24b.to_vec().as_mut());
    complete_data.append(data_prefix_chunk_12b.to_vec().as_mut());
    complete_data.append(data.to_vec().as_mut());


    // Some binary data  u8 -> 3, 3u8   hexa -> 0xaa   char bite -> b'a'
    //let data: [u8; DIM] = [86; DIM];
    /*let complete_data: [u8; DIM*5] = [
        0x52 , 0x49 , 0x46 , 0x46 , 0x46 , 0x2D , 0x00 , 0x00, 0x57 , 0x41 , 0x56 , 0x45 , 0x66 , 0x6D , 0x74 , 0x20,
        0x10 , 0x00 , 0x00 , 0x00 , 0x01 , 0x00 , 0x01 , 0x00, 0x22 , 0x56 , 0x00 , 0x00 , 0x22 , 0x56 , 0x00 , 0x00,
        0x01 , 0x00 , 0x08 , 0x00 , 0x64 , 0x61 , 0x74 , 0x61, 0x22 , 0x2D , 0x00 , 0x00 , 0x80 , 0x80 , 0x80 , 0x80,
        0x80 , 0x80 , 0x80 , 0x80 , 0x80 , 0x80 , 0x80 , 0x80, 0x80 , 0x80 , 0x80 , 0x80 , 0x80 , 0x80 , 0x80 , 0x80,
        0x80 , 0x80 , 0x80 , 0x80 , 0x80 , 0x80 , 0x80 , 0x80, 0x80 , 0x80 , 0x80 , 0x80 , 0x80 , 0x80 , 0x80 , 0x80
    ];*/
    /*
    246E:0100  52 49 46 46 46 2D 00 00-57 41 56 45 66 6D 74 20   RIFFF-..WAVEfmt
    246E:0110  10 00 00 00 01 00 01 00-22 56 00 00 22 56 00 00   ........"V.."V..
    246E:0120  01 00 08 00 64 61 74 61-22 2D 00 00 80 80 80 80   ....data"-......
    246E:0130  80 80 80 80 80 80 80 80-80 80 80 80 80 80 80 80   ................
    246E:0140  80 80 80 80 80 80 80 80-80 80 80 80 80 80 80 80   ................
    */
    println!("Decimal: {:?}", complete_data);
    println!("Hexadecimal: {:X?}", complete_data);
    println!("Hexadecimal with 0s: {:02X?}", complete_data);
    print_data_exa_and_ascii(&complete_data);

    /*let data = b"hello";
    // lower case
    println!("{:x?}", data);
    // upper case
    println!("{:X?}", data);

    let data = [0x0, 0x1, 0xe, 0xf, 0xff];
    // print the leading zero
    println!("{:02X?}", data);
    // It can be combined with the pretty modifier as well
    println!("{:#04X?}", data);*/
    // Write binary data to the file
    file.write_all(&complete_data)?;

    println!("Binary data written to binary_output.bin");
    Ok(())
}
fn generate_wave(data: &mut [u8], dim: usize){
    for i in 0..dim {
        let modi:i32= (i % 22) as i32;
        if modi<11 {
            data[i]= (128+((modi - 11)*10)) as u8;
        }
        if modi>=11 {
            data[i]= (128+((modi - 11)*10)) as u8;
        }
    }
}
fn print_data_exa_and_ascii(data: &[u8]) {
    let len = data.len();
    let mut rows =len/8;
    if len % 8 != 0 {
        rows += 1;
    }
    for i in 0..rows {
        for j in 0..8 {
            if (i*8 + j)<len{
                print!("{:02X} ", data[i*8 + j]);
            } else {
                print!("  ");
            }
        }
        print!("\t");
        /*let vec=&Vec::from(data)[i..i+8];
        for element in vec {
            print!("{:02X} ", element);
        }*//*

        fix display!!!
        let vec=&Vec::from(data)[i*8..(i*8)+8];
        let s = String::from_utf8_lossy(vec);
        s.chars().for_each(|c| print!("{} ", c));*/
        //print!("{}", s);
        println!();
    }
    println!();
}
/*fn pause_input_wait(){
    use std::io;
    println!("Press Enter to exit...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}
*/
/*append to a file



use std::fs::OpenOptions;
use std::io::Write;

fn main() -> std::io::Result<()> {
    // Open the file in append mode
    let mut file = OpenOptions::new()
        .append(true)
        .open("output.txt")?;

    // Write additional data to the file
    file.write_all(b"\nAppending more data!")?;

    println!("Data appended to output.txt");
    Ok(())
}


Error Handling: Use match or the ? operator to handle errors.
File Path: Replace "output.txt" with your desired file path.
Binary Files: For binary data, use write_all with byte slices.*/