extern crate tokio;
extern crate bytes;

use tokio::runtime::Builder;

mod configuration;
mod controlserver;
mod clienthandler;
mod udphandler;
mod protos;
mod util;

use std::path::Path;

fn main() {
    configuration::init(Path::new("config.toml"));
    configure_logging();
    let config = &configuration::cfg().runtime;
    let runtime = Builder::new_multi_thread()
        .worker_threads(config.threads)
        .thread_name(&config.thread_name)
        .thread_stack_size(config.thread_stack)
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(async {
        let server = controlserver::ControlServer::new();
        server.run().await.unwrap();
    });
}

fn configure_logging() {
    let logging_config = &configuration::cfg().logging;
    tracing_subscriber::fmt()
        .with_max_level(logging_config.level)
        .init();
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    extern crate hound;

    use std::fs::File;
    use std::cmp;
    use std::fs;
    use hound::{WavReader, WavWriter};
    use audiopus::coder::Encoder as OpusEncoder;
    use audiopus::coder::Decoder as OpusDecoder;
    use audiopus::{Application, Channels, SampleRate};

    #[test]
    fn testrecode() {
        const SAMPLE_SIZE: usize = 120;
        let _ = fs::remove_file("target/recoded.wav");
        let mut reader = WavReader::open("resources/original.wav").unwrap();
        let mut writer = WavWriter::new(File::create("target/recoded.wav").expect("OK"), reader.spec().clone()).unwrap();
        println!("{:?}", reader.spec());
        let samples = reader.samples::<i16>();
        let iterations = samples.len() / SAMPLE_SIZE;

        println!("Unencoded Size: {}", samples.len() * 2);
        let input: Vec<i16> = samples.map(|res| res.unwrap()).collect();
        let mut output: [u8; 8192] = [0; 8192];
        let mut recoded: [i16; 8192] = [0; 8192];
        let encoder = OpusEncoder::new(
            SampleRate::Hz48000,
            Channels::Mono,
            Application::Voip,
        ).expect("BAH !!");
        let mut decoder = OpusDecoder::new(
            SampleRate::Hz48000,
            Channels::Mono,
        ).expect("BAH !!");
        let mut codesize = 0;
        let mut max_size = 0;
        let mut min_size = 10000;
        for i in 0..iterations {
            let offset = i * SAMPLE_SIZE;
            let encoded = match encoder.encode(&input[offset..offset + SAMPLE_SIZE], &mut output) {
                Ok(size) => size,
                Err(e) => {
                    println!("Error: {}", e);
                    return ();
                }
            };
            codesize = codesize + encoded;
            max_size = cmp::max(max_size, encoded);
            min_size = cmp::min(min_size, encoded);
            let recodesize = match decoder.decode(Some(&output[0..encoded]), &mut recoded[..], false) {
                Ok(size) => size,
                Err(e) => {
                    println!("Error: {}", e);
                    return ();
                }
            };
            let mut swriter = writer.get_i16_writer(recodesize as u32);
            for sample in recoded[0..recodesize].iter() {
                swriter.write_sample(*sample);
            }
            swriter.flush().expect("Unable to write output !");
        }
        println!("Encoded size: {}, Iterations: {}", codesize, iterations);
        println!("Max Size Packet: {}, Min Size Packet: {}", max_size, min_size);
    }
}

