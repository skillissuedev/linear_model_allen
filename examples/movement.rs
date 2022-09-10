use std::{f32::consts::PI, thread, time::Duration};

use allen::{Buffer, BufferData, Channels, Device, Source};

const HERTZ: f32 = 1200.0;
const SAMPLE_RATE: i32 = 44100;

fn main() {
    let device = Device::open(None).unwrap();

    let context = device.create_context().unwrap();
    context.make_current();

    assert!(context.is_current());

    let buffer = Buffer::new().unwrap();
    let source = Source::new().unwrap();

    // Generate sine waves.
    let data = (0..SAMPLE_RATE)
        .map(|i| {
            ((2.0 * PI * HERTZ * i as f32 / SAMPLE_RATE as f32).sin() * i16::MAX as f32) as i16
        })
        .collect::<Vec<_>>();

    buffer
        .data(BufferData::I16(data), Channels::Mono, SAMPLE_RATE)
        .unwrap();

    source.set_buffer(&buffer);
    source.set_looping(true);
    source.play().unwrap();

    println!("{:?}", source.position());

    let mut timer = 0u32;

    loop {
        thread::sleep(Duration::from_secs_f32(1.0 / 60.0));

        timer += 1;

        let timer = timer as f32 * 0.05;

        source.set_position([timer.sin() * 100.0, 0.0, timer.cos() * 100.0]);
    }
}