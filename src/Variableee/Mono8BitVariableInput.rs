use std::f64::consts::PI;
use std::fs::File;
use std::io;
use std::io::{Write};
const WAV_FILENAME: &str = "Mono8BitVariableInput.wav";
//standard cd sample rate 44100 samples per second, Video at 48000 samples per second
const SAMPLE_RATE: u32 = 44100;
//HEADER_SIZE in bytes
const HEADER_SIZE: u32 = 44;

fn main() {
    create_binary_file().expect("TODO: panic message");
}
fn create_binary_file() -> std::io::Result<()>{
    let mut file = File::create(WAV_FILENAME)?;
    //duration of RIFF file in seconds
    let duration_sec=read_duration_sec();
    //Frequency of wave wanted
    let frequency_of_wave=read_frequency_of_wave();
    //size of file in bytes, *1 is multiplying by the size of the BitsPerSample translated in bytes
    let size: u32 = HEADER_SIZE+(duration_sec*SAMPLE_RATE*1);



    //Resource Interchange File Format.
    const RIFF: [u8; 4] = [b'R', b'I', b'F', b'F'];
    //which is the length of the entire file minus the 8 bytes for the "RIFF" and file_len_at_riff
    let file_len_at_riff: Vec<u8> = (size -8).to_le_bytes().to_vec();
    const WAVE: [u8; 4] = [b'W', b'A', b'V', b'E'];
    //assembly of first chunk
    let riff_chunk_12b: Vec<u8> = [RIFF.to_vec(), file_len_at_riff, WAVE.to_vec()].concat();

    //format chunk
    const FMT_: [u8; 4] = [b'f', b'm', b't', b' '];
    //format Chunk size minus 8 bytes=(0x10) because we have PCM
    let bloc_size: Vec<u8> = vec![0x10, 0x0, 0x0, 0x0];
    // In a PCM stream, amplitude is sampled at regular intervals and the sample is quantized
    //at the closest value in a set of digital steps that divide the amplitude
    // is PCM = 1 (i.e. Linear quantization) Values other than 1 indicate some form of compression.
    let audio_format: Vec<u8> = vec![0x01, 0x00];
    //Number of channels   Mono = 1, Stereo = 2, etc.  is there 2+ channels?
    let number_of_channels: Vec<u8> = vec![0x01, 0x00];
    //Sample rate (in hertz)
    let sample_rate: Vec<u8> = SAMPLE_RATE.to_le_bytes().to_vec();
    //Number of bytes to read per second (SampleRate * BytePerBloc).
    //equals to SampleRate * NumChannels * BitsPerSample/8
    let byte_per_second: Vec<u8> = (SAMPLE_RATE*1*1).to_le_bytes().to_vec();
    // Number of bytes per block (NbrChannels * BitsPerSample / 8)
    // The number of bytes for one sample including all channels.
    // I wonder what happens when this number isn't an integer?
    let byte_per_bloc: Vec<u8> = vec![0x01, 0x00];
    //8-bit samples are stored as unsigned bytes, ranging from 0 to 255.
    // 16-bit samples are stored as 2's-complement signed integers, ranging from -32768 to 32767.
    let bits_per_sample: Vec<u8> = vec![0x08, 0x00];
    //assembly of format chunk
    let format_chunk_24b: Vec<u8> = [FMT_.to_vec(), bloc_size, audio_format, number_of_channels, sample_rate, byte_per_second, byte_per_bloc, bits_per_sample].concat();

    const DATA: [u8; 4] = [b'd', b'a', b't', b'a'];
    //which is the length of the entire file minus the 44 bytes for Header comprised of the file_len_at_data 4 bites
    // equals to NumSamples * NumChannels * BitsPerSample/8
    let file_len_at_data: Vec<u8> = (size -44).to_le_bytes().to_vec();
    //data vector
    let data: Vec<u8> = generate_wave(size - 44, frequency_of_wave);
    //assembly of data chunk
    let data_chunk_12b: Vec<u8> = [DATA.to_vec(), file_len_at_data, data].concat();

    //assembly of entire file
    let complete_data = [riff_chunk_12b, format_chunk_24b, data_chunk_12b].concat();

    //print of entire file
    print_data_exa_and_ascii(&complete_data);

    // write on file
    file.write_all(&complete_data)?;
    // conformation of data writing
    println!("Binary data written to {} of size {} bytes", WAV_FILENAME, size);
    Ok(())
}
//this reads a 32 bit unsigned inter, represents the read_duration_sec
fn read_duration_sec() -> u32 {
    println!("Enter the duration of the wav file in seconds (4 byte u32 from 0 to 4294967295)");
    let mut input = String::new();
    input= read(input);
    let output: u32 = input.trim().parse().expect("Input not an integer of 4 bytes");
    output
}
//this reads a 32 bit unsigned inter, represents the read_frequency_of_wave
fn read_frequency_of_wave() -> u32 {
    println!("Enter the frequency of the wave in Hz (4 byte u32 from 0 to 4294967295)");
    let mut input = String::new();
    input= read(input);
    let output: u32 = input.trim().parse().expect("Input not an integer of 4 bytes");
    output
}
//this reads a string
fn read(mut input: String)->String{
    match io::stdin().read_line(&mut input) {
        Ok(..) => input,
        Err(error) => { println!("error: {error}");
            error.to_string()
        },
    }
}
//this generates a sine wave from the frequency_of_wave and it feeds the sampled values in the data vector
fn generate_wave(length: u32, frequency_of_wave: u32) -> Vec<u8>{
    let mut data: Vec<u8> = Vec::with_capacity(length as usize);

    //y(t) = A sin(2pi freqOfWave t + initial phase)
    //t not continuous so t= n periodOfOneSample=n/SampleRate
    //y(t) = A sin(2pi freqOfWave n/SampleRate + initial phase)
    // we isolate n and discard initial phase=0
    //y(t) = A sin(n (2pi freqOfWave/SampleRate))
    //delta_angle_per_increment=(2pi freqOfWave/SampleRate)
    //y(t) = A sin(n delta_angle_per_increment)
    let delta_angle_per_increment =2_f64*PI*(frequency_of_wave as f64/SAMPLE_RATE as f64);
    let mut phase =0_f64;

    for _ in 0..length {
        data.push(((phase.sin()+1_f64)*(255_f64/2_f64)) as u8);
        phase=phase+delta_angle_per_increment;
    }
    data
}
fn print_data_exa_and_ascii(data: &Vec<u8>) {
    for i in (0..data.len()).step_by(8) {
        for j in 0..8 {
            if i+j < data.len() {
                print!("{:02X} ", data[i+j]);
            }else {
                print!(".  ");
            }
        }
        print!("\t");
        for j in 0..8 {
            if i+j < data.len() && data[i+j]>31 && data[i+j]!=127 && data[i+j]!=127 && data[i+j]!=129 && data[i+j]!=141 && data[i+j]!=143 && data[i+j]!=144{
                print!("{} ",  data[i+j] as char);
            }else {
                print!(".  ");
            }
        }
        println!();
    }
}
