use std::fs::File;
use std::io;
use std::io::{Write};

const WAV_FILENAME: &str = "parametric_wav.wav";

fn main() {
    create_binary_file().expect("TODO: panic message");
}
fn create_binary_file() -> std::io::Result<()>{
    let mut file = File::create(WAV_FILENAME)?;
    const DIM: usize =65491;
    let size: u32 = read_4byte_u32();

    const RIFF: [u8; 4] = [b'R', b'I', b'F', b'F'];
    // (4 bytes) : Overall file size minus 8 bytes
    // let file_len_at_riff: Vec<u8> = vec![0xF7, 0xff, 0x00, 0x00];/*which is the length of the entire file minus the 8 bytes for the "RIFF" and length (11598 - 11590 = 8 bytes), little endian real length= 255+8b of "RIFF" and length ---> 263 bytes*/
    let file_len_at_riff: Vec<u8> = (size -8).to_le_bytes().to_vec();/*which is the length of the entire file minus the 8 bytes for the "RIFF" and length (11598 - 11590 = 8 bytes), little endian real length= 255+8b of "RIFF" and length ---> 263 bytes*/
    const WAVE: [u8; 4] = [b'W', b'A', b'V', b'E'];

    let riff_chunk_12b: Vec<u8> = [RIFF.to_vec(), file_len_at_riff, WAVE.to_vec()].concat();

    const FMT_: [u8; 4] = [b'f', b'm', b't', b' '];
    let bloc_size: Vec<u8> = vec![0x10, 0x0, 0x0, 0x0];/*(4 bytes) : Chunk size minus 8 bytes, which is 16 bytes here  (0x10)*/
    let audio_format: Vec<u8> = vec![0x01, 0x00];/*(2 bytes) : Audio format (1: PCM integer, 3: IEEE 754 float)*/
    let number_of_channels: Vec<u8> = vec![0x01, 0x00];/*(2 bytes) : Number of channels*/
    let frequency: Vec<u8> = vec![0x22, 0x56, 0x00, 0x00];/*(4 bytes) : Sample rate (in hertz)*/
    let byte_per_second: Vec<u8> = vec![0x22, 0x56, 0x00, 0x00];/*(4 bytes) : Number of bytes to read per second (Frequency * BytePerBloc).*/
    let byte_per_bloc: Vec<u8> = vec![0x01, 0x00];/*(2 bytes) : Number of bytes per block (NbrChannels * BitsPerSample / 8).*/
    let bits_per_sample: Vec<u8> = vec![0x08, 0x00];/*(2 bytes) : Number of bits per sample*/

    let format_chunk_24b: Vec<u8> = [FMT_.to_vec(), bloc_size, audio_format, number_of_channels, frequency, byte_per_second, byte_per_bloc, bits_per_sample].concat();

    const DATA: [u8; 4] = [b'd', b'a', b't', b'a'];/*(4 bytes) : Identifier « data »  (0x64, 0x61, 0x74, 0x61)*/
    // let file_len_at_data: Vec<u8> = vec![0xD3, 0xFF, 0x00, 0x00];/*(4 bytes) : SampledData size   Length Of Data To Follow small endian 263  real length= 219+44b of "RIFF" and length ---> 263 bytes*/
    let file_len_at_data: Vec<u8> = (size -44).to_le_bytes().to_vec();/*(4 bytes) : SampledData size   Length Of Data To Follow small endian 263  real length= 219+44b of "RIFF" and length ---> 263 bytes*/
    // let mut data: [u8; (SIZE - 44) as usize] = [0x80; (SIZE - 44) as usize];
    let mut data: Vec<u8> = vec![0x80; (size - 44) as usize];
    generate_wave(&mut data, (size - 44) as usize);

    let data_chunk_12b: Vec<u8> = [DATA.to_vec(), file_len_at_data, data].concat();


    let complete_data = [riff_chunk_12b, format_chunk_24b, data_chunk_12b].concat();

    /*println!("Decimal: {:?}", complete_data);
    println!("Hexadecimal: {:X?}", complete_data);
    println!("Hexadecimal with 0s: {:02X?}", complete_data);*/
    print_data_exa_and_ascii(&complete_data);


    file.write_all(&complete_data)?;
    println!("Binary data written to {}", WAV_FILENAME);
    Ok(())
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
        for j in 0..8 {
            if (i*8 + j)<len{
                print!("{} ",  data[i*8 + j] as char);
            } else {
                print!("  ");
            }
        }

        /*let vec=&Vec::from(data)[i..i+8];
        for element in vec {
            print!("{:02X} ", element);
        }*/



        //fix display!!!
        /*let vec=&Vec::from(data)[i*8..(i*8)+8];
        let s = String::from_utf8_lossy(vec);
        s.chars().for_each(|c| print!("{} ", c));*/
        //print!("{}", s);
        println!();
    }
    println!();
    println!("{}", len)
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