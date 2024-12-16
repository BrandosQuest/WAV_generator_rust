use std::f64::consts::PI;
use std::fs::File;
use std::io;
use std::io::{Write};
/*f you're recording music, a standard sample rate is 44.1 kHz or 44,100 samples per second. This is the standard for most consumer audio, used for formats like CDs. 48 kHz is another common audio sample rate used for movies.*/
const WAV_FILENAME: &str = "variableFreq.wav";
const SAMPLE_RATE: u32 = 44100;// a standard sample rate is 44.1 kHz or 44,100 samples per second.
const HEADER_SIZE: u32 = 44;//HEADER_SIZE in bytes

fn main() {
    create_binary_file().expect("TODO: panic message");
}
fn create_binary_file() -> std::io::Result<()>{
    let duration_sec=read_duration_sec();
    let frequency_of_wave=read_frequency_of_wave();
    // let size: u32 = read_4byte_u32();
    let size: u32 = HEADER_SIZE+(duration_sec*SAMPLE_RATE*1);//*1 is multiplying by the size of the BitsPerSample translated in bytes
    println!("Size of file is {}", size);
    let mut file = File::create(WAV_FILENAME)?;


    const RIFF: [u8; 4] = [b'R', b'I', b'F', b'F'];//Resource Interchange File Format.
    // (4 bytes) : Overall file size minus 8 bytes
    let file_len_at_riff: Vec<u8> = (size -8).to_le_bytes().to_vec();/*which is the length of the entire file minus the 8 bytes for the "RIFF" and length (11598 - 11590 = 8 bytes), little endian real length= 255+8b of "RIFF" and length ---> 263 bytes*/
    const WAVE: [u8; 4] = [b'W', b'A', b'V', b'E'];
    let riff_chunk_12b: Vec<u8> = [RIFF.to_vec(), file_len_at_riff, WAVE.to_vec()].concat();

    const FMT_: [u8; 4] = [b'f', b'm', b't', b' '];
    let bloc_size: Vec<u8> = vec![0x10, 0x0, 0x0, 0x0];/*(4 bytes) : Chunk size minus 8 bytes, which is 16 bytes here  (0x10) because we have PCM Length Of FORMAT Chunk (Binary, always 0x10)*/
    // In un flusso PCM, l'ampiezza del segnale analogico viene campionata a intervalli uniformi e ciascun campione viene quantizzato al valore più vicino entro un intervallo di steps digitali.
    let audio_format: Vec<u8> = vec![0x01, 0x00];/*(2 bytes) : Audio format (1: PCM integer, 3: IEEE 754 float)   PCM = 1 (i.e. Linear quantization) Values other than 1 indicate some form of compression.*/
    let number_of_channels: Vec<u8> = vec![0x01, 0x00];/*(2 bytes) : Number of channels   Mono = 1, Stereo = 2, etc.  is there 2+ channels?*/
    // let sample_rate: Vec<u8> = vec![0x22, 0x56, 0x00, 0x00];/*(4 bytes) : Sample rate (in hertz)*/
    let sample_rate: Vec<u8> = SAMPLE_RATE.to_le_bytes().to_vec();/*(4 bytes) : Sample rate (in hertz)*/
    // let byte_per_second: Vec<u8> = vec![0x22, 0x56, 0x00, 0x00];/*(4 bytes) : Number of bytes to read per second (Frequency * BytePerBloc). == SampleRate * NumChannels * BitsPerSample/8*/
    let byte_per_second: Vec<u8> = (SAMPLE_RATE*1*1).to_le_bytes().to_vec();/*(4 bytes) : Number of bytes to read per second (Frequency * BytePerBloc). == SampleRate * NumChannels * BitsPerSample/8*/
    let byte_per_bloc: Vec<u8> = vec![0x01, 0x00];/*(2 bytes) : Number of bytes per block (NbrChannels * BitsPerSample / 8).    == NumChannels * BitsPerSample/8
                               The number of bytes for one sample including
                               all channels. I wonder what happens when
                               this number isn't an integer?*/
    let bits_per_sample: Vec<u8> = vec![0x08, 0x00];/*(2 bytes) : Number of bits per sample*///8-bit samples are stored as unsigned bytes, ranging from 0 to 255. 16-bit samples are stored as 2's-complement signed integers, ranging from -32768 to 32767.

    let format_chunk_24b: Vec<u8> = [FMT_.to_vec(), bloc_size, audio_format, number_of_channels, sample_rate, byte_per_second, byte_per_bloc, bits_per_sample].concat();

    const DATA: [u8; 4] = [b'd', b'a', b't', b'a'];/*(4 bytes) : Identifier « data »  (0x64, 0x61, 0x74, 0x61)*/
    // let file_len_at_data: Vec<u8> = vec![0xD3, 0xFF, 0x00, 0x00];/*(4 bytes) : SampledData size   Length Of Data To Follow small endian 263  real length= 219+44b of "RIFF" and length ---> 263 bytes*/
    let file_len_at_data: Vec<u8> = (size -44).to_le_bytes().to_vec();/*(4 bytes) : SampledData size   Length Of Data To Follow small endian 263  real length= 219+44b of "RIFF" and length ---> 263 bytes
    == NumSamples * NumChannels * BitsPerSample/8
                               This is the number of bytes in the data.
                               You can also think of this as the size
                               of the read of the subchunk following this
                               number.*/

    // let mut data: [u8; (SIZE - 44) as usize] = [0x80; (SIZE - 44) as usize];
    let mut data: Vec<u8> = vec![0x80; (size - 44) as usize];
    generate_wave(&mut data, frequency_of_wave);

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
fn read_duration_sec() -> u32 {
    println!("Enter the duration of the wav file in seconds (4 byte u32 from 0 to 4294967295)");
    let mut input = String::new();
    input= read(input);
    let output: u32 = input.trim().parse().expect("Input not an integer of 4 bytes");
    //println!("output: {}", output);
    return output;
}
fn read_frequency_of_wave() -> u32 {
    println!("Enter the frequency of the wave in Hz (4 byte u32 from 0 to 4294967295)");
    let mut input = String::new();
    input= read(input);
    let output: u32 = input.trim().parse().expect("Input not an integer of 4 bytes");
    //println!("output: {}", output);
    return output;
}
fn read_4byte_u32()-> u32 {
    println!("Enter the size of the wav file(4 byte u32 from 44 to 4294967295)");
    let mut input = String::new();
    let mut b=true;
    let mut output:u32=0;
    while b {
        input= read(input);
        output = input.trim().parse().expect("Input not an integer of 4 bytes");
        if output > 44 {
            b=false;
        }
    }

    println!("output: {}", output);
    return output;

}
fn read(mut input: String)->String{
    match io::stdin().read_line(&mut input) {
        Ok(..) => {//println!("{input}");
            //println!("{:?}", input.as_bytes());
            input}
        Err(error) => { println!("error: {error}");
            error.to_string()
        },
    }
}
fn generate_wave(data: &mut [u8], frequency_of_wave: u32){
    use std::convert::TryInto;

    let length: u32 = data.len().try_into().unwrap();



    //y(t) = A sin(2pi freqOfWave t + initial phase)
    //t not continous so t= n periodOfOneSample=n/SampleRate
    //y(t) = A sin(2pi freqOfWave n/SampleRate + initial phase)
    // we isolate n and discard initial phase=0
    //y(t) = A sin(n (2pi freqOfWave/SampleRate))
    //delta_angle_per_increment=(2pi freqOfWave/SampleRate)
    //y(t) = A sin(n delta_angle_per_increment)
    let delta_angle_per_increment =2_f64*PI*(frequency_of_wave as f64/SAMPLE_RATE as f64);
    let mut phase =0_f64;

    for i in 0..length {
        data[i as usize]= ((phase.sin()+1_f64)*(255_f64/2_f64)) as u8;
        phase=phase+delta_angle_per_increment;

        /*THIS WORKS
        let delta_angle_per_increment =2_f64*PI*(frequency_of_wave as f64/SAMPLE_RATE as f64);

        for i in 0..length {
            data[i as usize]= (((delta_angle_per_increment * (i as f64)).sin()+1_f64)*(255_f64/2_f64)) as u8;*/


        /*    THIS KINDA WORKS
        let length: u32 = data.len().try_into().unwrap();
        let samples_per_wavelength: u32 = SAMPLE_RATE/frequency_of_wave;
        let mut scaled_x_values: f64;

        for loop

        scaled_x_values= ((i%samples_per_wavelength)as f64*(360_f64/samples_per_wavelength as f64))as f64;
        println!("scaled_x_values: {}", scaled_x_values);
        //println!("samples_per_wavelength: {}", samples_per_wavelength);
        println!("(scaled_x_values.sin(): {}", scaled_x_values.to_radians().sin());
        data[i as usize]=((scaled_x_values.to_radians().sin()+1_f64)*(255_f64/2_f64)) as u8;
        //println!("data[i as usize]: {}", samples_per_wavelength);*/



        //let one = f64::sin(1.5763268);
        /*
        let wave = (f64::sin(i as f64/1_f64)*128_f64)as u8;
        let positive_wave = wave+128;
        data[i]=positive_wave;*/
        /*let modi:i32= (i % 22) as i32;
        if modi<11 {
            data[i]= (128+((modi - 11)*10)) as u8;
        }
        if modi>=11 {
            data[i]= (128+((modi - 11)*10)) as u8;
        }*/
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
            if (i*8 + j)<len && data[i*8 + j]>31{
                print!("{}.",  data[i*8 + j] as char);
            } else {
                print!(" .");
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
        /* print!("{}", s); */
        println!();
    }
    println!();
    println!("Size of file is {}", len);
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