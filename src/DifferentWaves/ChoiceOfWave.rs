use std::f64::consts::PI;
use std::fs::File;
use std::{io, vec};
use std::io::{Write};
const WAV_FILENAME: &str = "ChoiceOfWave.wav";
//standard cd sample rate 44100 samples per second, Video at 48000 samples per second
const SAMPLE_RATE: u32 = 48000;
//HEADER_SIZE in bytes
const HEADER_SIZE: u32 = 44;
#[derive(Debug)]
enum TypeOfWave {
    Sinusoidal,
    Sawtooth,
    Square,
    Triangle,
}

fn main() -> std::io::Result<()> {
    //file to save wave
    let mut file = File::create(WAV_FILENAME)?;



    //assembly of entire file
    let complete_data = generate_file_wave_vec(wave_choice_input());

    //print of entire file
    print_data_exa_and_ascii(&complete_data);

    // write on file
    file.write_all(&complete_data)?;

    // conformation of data writing
    println!("Binary data written to {}", WAV_FILENAME);
    Ok(())
}
fn wave_choice_input() -> Vec<TypeOfWave> {
    println!("Wave choice: Sinusoidal, Sawtooth, Square, Triangle. (or any combination separated by a space or numbers values)");
    let mut input = String::new();
    input= read(input);

    let parts = input.split_whitespace();
    let mut split_input: Vec<String> = Vec::new();
    for part in parts {
        split_input.push(part.trim().to_string().to_ascii_lowercase());
    }
    let mut waves_kinds: Vec<TypeOfWave> = Vec::new();
    for splitting in split_input {
        match splitting.as_str() {
            "1" => waves_kinds.push(TypeOfWave::Sinusoidal),
            "2" => waves_kinds.push(TypeOfWave::Sawtooth),
            "3" => waves_kinds.push(TypeOfWave::Square),
            "4" => waves_kinds.push(TypeOfWave::Triangle),
            "sinusoidal" => waves_kinds.push(TypeOfWave::Sinusoidal),
            "sawtooth" => waves_kinds.push(TypeOfWave::Sawtooth),
            "square" => waves_kinds.push(TypeOfWave::Square),
            "triangle" => waves_kinds.push(TypeOfWave::Triangle),
            _ => println!("Input does not equal any value"),
        }
    }
    println!("{:?}", waves_kinds);

    waves_kinds
}

fn generate_file_wave_vec(waves_kinds: Vec<TypeOfWave>) -> Vec<u8> {
    let duration_sec=read_duration_sec();
    let frequency_of_wave=read_frequency_of_wave();
    let bits_per_sample_u16:u16 = 16;
    let number_of_channels_u16:u16 = 2;
    //size of file in bytes, *1 is multiplying by the size of the BitsPerSample translated in bytes
    let size: u32 = HEADER_SIZE+(duration_sec*SAMPLE_RATE*(bits_per_sample_u16 as u32)*(number_of_channels_u16 as u32)/8);

    //Resource Interchange File Format
    const RIFF: [u8; 4] = [b'R', b'I', b'F', b'F'];
    //which is the length of the entire file minus the 8 bytes for the "RIFF" and file_len_at_riff
    let file_len_at_riff: Vec<u8> = (size -8).to_le_bytes().to_vec();
    const WAVE: [u8; 4] = [b'W', b'A', b'V', b'E'];
    let riff_chunk_12b: Vec<u8> = [RIFF.to_vec(), file_len_at_riff, WAVE.to_vec()].concat();

    const FMT_: [u8; 4] = [b'f', b'm', b't', b' '];
    //format Chunk size minus 8 bytes=(0x10) because we have PCM
    let bloc_size: Vec<u8> = vec![0x10, 0x0, 0x0, 0x0];
    // In a PCM stream, amplitude is sampled at regular intervals and the sample is quantized
    // at the closest value in a set of digital steps that divide the amplitude
    // is PCM = 1 (i.e. Linear quantization) Values other than 1 indicate some form of compression.
    let audio_format: Vec<u8> = vec![0x01, 0x00];
    let number_of_channels: Vec<u8> = number_of_channels_u16.to_le_bytes().to_vec();//let number_of_channels: Vec<u8> = vec![0x01, 0x00];
    let sample_rate: Vec<u8> = SAMPLE_RATE.to_le_bytes().to_vec();
    let byte_per_second: Vec<u8> = (SAMPLE_RATE*number_of_channels_u16 as u32*bits_per_sample_u16 as u32/8).to_le_bytes().to_vec();
    let byte_per_bloc: Vec<u8> = (number_of_channels_u16*bits_per_sample_u16/8).to_le_bytes().to_vec();
    // 16-bit samples are stored as 2's-complement signed integers, ranging from -32768 to 32767.
    let bits_per_sample: Vec<u8> = bits_per_sample_u16.to_le_bytes().to_vec();
    let format_chunk_24b: Vec<u8> = [FMT_.to_vec(), bloc_size, audio_format, number_of_channels, sample_rate, byte_per_second, byte_per_bloc, bits_per_sample].concat();

    const DATA: [u8; 4] = [b'd', b'a', b't', b'a'];
    //which is the length of the entire file minus the 44 bytes for Header comprised of the file_len_at_data 4 bites
    let file_len_at_data: Vec<u8> = (size -44).to_le_bytes().to_vec();
    let data: Vec<u8> = generate_wave(size - 44, frequency_of_wave, waves_kinds);
    let data_chunk_12b = [DATA.to_vec(), file_len_at_data, data].concat();

    [riff_chunk_12b, format_chunk_24b, data_chunk_12b].concat()
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
fn generate_wave(length_bytes: u32, frequency_of_wave: u32, waves_kinds: Vec<TypeOfWave>) -> Vec<u8>{
    for wave_kind in waves_kinds{
        match wave_kind {
            TypeOfWave::Sinusoidal => waves_kinds.push(TypeOfWave::Sinusoidal),
            TypeOfWave::Sawtooth => waves_kinds.push(TypeOfWave::Sawtooth),
            TypeOfWave::Square => waves_kinds.push(TypeOfWave::Square),
            TypeOfWave::Triangle => waves_kinds.push(TypeOfWave::Triangle),
        }
    }
    let mut data: Vec<u8> = Vec::with_capacity(length_bytes as usize);


    //y(n) = 2A ((n (frequency_of_wave/SAMPLE_RATE)) mod1) - A
    // A: Amplitude of the wave (e.g., max absolute value).
    // frequency_of_wave: Frequency of the wave (in Hz, cycles per second).
    // SAMPLE_RATE: Sample rate (in Hz, samples per second).
    // n: Current sample index (0, 1, 2, ...).
    //t not continuous so t= n periodOfOneSample=n/SampleRate
    let delta_phase_per_increment =frequency_of_wave as f64/SAMPLE_RATE as f64;
    let mut phase = 0_f64;
    let amplitude:f64 = (2_i64.pow(16)/2) as f64;

    // let mut counter=0;
    for _ in 0..(length_bytes/2)/2 {
        let sample_left = ((2_f64*amplitude*phase)-amplitude) as i16;
        data.push(sample_left.to_le_bytes()[0]);
        data.push(sample_left.to_le_bytes()[1]);

        let sample_right = ((2_f64*amplitude*phase)-amplitude) as i16;
        data.push(sample_right.to_le_bytes()[0]);
        data.push(sample_right.to_le_bytes()[1]);

        phase=(phase+delta_phase_per_increment)%1.0;
        // phase=(counter as f64*delta_phase_per_increment)%1.0;
        // counter+=1;
    }
    /*//y(t) = A sin(2pi freqOfWave t + initial phase)
    //t not continuous so t= n periodOfOneSample=n/SampleRate
    //y(t) = A sin(2pi freqOfWave n/SampleRate + initial phase)
    // we isolate n and discard initial phase=0
    //y(t) = A sin(n (2pi freqOfWave/SampleRate))
    //delta_angle_per_increment=(2pi freqOfWave/SampleRate)
    //y(t) = A sin(n delta_angle_per_increment)
    let delta_angle_per_increment =2_f64*PI*(frequency_of_wave as f64/SAMPLE_RATE as f64);
    let mut phase =0_f64;
    let amplitude:f64 = (2_i64.pow(16)/2) as f64;

    for _ in 0..(length_bytes/2)/2 {
        let sample_left = (phase.sin()*amplitude) as i16;
        //println!("phase: {}", phase);
        // println!("sample: {}", sample_left);
        // println!("amplitude: {}", amplitude);
        data.push(sample_left.to_le_bytes()[0]);
        data.push(sample_left.to_le_bytes()[1]);

        // let sample_right = ((phase+PI).sin()*amplitude) as i16;
        let sample_right = (phase.sin()*amplitude) as i16;
        data.push(sample_right.to_le_bytes()[0]);
        data.push(sample_right.to_le_bytes()[1]);

        phase=phase+delta_angle_per_increment;
    }*/
    data

}
fn print_data_exa_and_ascii(data: &Vec<u8>) {
    for i in (0..data.len()).step_by(8) {
        // for i in (0..200).step_by(8) {
        for j in 0..8 {
            if i+j < data.len() {
                print!("{:02X} ", data[i+j]);
            }else {
                print!(".  ");
            }
        }
        print!("\t");
        for j in 0..8 {
            if i+j < data.len() && data[i+j]>31 && data[i+j]!=127 && data[i+j]<127{
                print!("{} ",  data[i+j] as char);
            }else {
                print!(". ");
            }
        }
        print!("\t");
        for j in (0..8).step_by(2) {
            if i+j < data.len() {
                print!("{:<6} ",  i16::from_le_bytes([data[i+j], data[i+j+1]]));
            }else {
                print!(".      ");
            }
        }
        println!();
    }
}
