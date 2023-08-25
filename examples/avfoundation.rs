extern crate ffmpeg_next as ffmpeg;
use ffmpeg::{
    media::Type,
    codec,
    encoder,
    format,
};

fn main() -> Result<(), ffmpeg::Error> {
    ffmpeg::init().unwrap();
    let mut input_options = ffmpeg::util::dictionary::Owned::new();
    let input_device = if let Some(device) = ffmpeg::device::input::video().next() {
        input_options.set("framerate", "30");
        input_options.set("probesize", "100M");
        input_options.set("video_size", "1920x1080");
        input_options.set("pixel_format", "uyvy422");
        device
    } else {
        return Ok(());
    };
    let video_path = std::path::Path::new("default");
    let ctx = ffmpeg::format::open_with(&video_path, &input_device, input_options).unwrap();
    let input = ctx.input();
    let input_stream = input.streams().best(Type::Video).ok_or(ffmpeg::Error::StreamNotFound).unwrap();
    const OUTPUT_FILE_NAME: &str = "test.mp4";
    let output_path = std::path::Path::new(OUTPUT_FILE_NAME);
    let mut octx = ffmpeg::format::output(&output_path).unwrap();
    let mut ost = octx.add_stream(encoder::find(codec::Id::H264))?;
    let decoder = ffmpeg::codec::context::Context::from_parameters(input_stream.parameters())?
        .decoder()
        .video()?;
    let mut encoder = codec::context::Context::from_parameters(ost.parameters())?
        .encoder()
        .video()?;
    encoder.set_height(decoder.height());
    encoder.set_width(decoder.width());
    encoder.set_aspect_ratio(decoder.aspect_ratio());
    encoder.set_format(decoder.format());
    encoder.set_frame_rate(decoder.frame_rate());
    //encoder.set_time_base(decoder.frame_rate().unwrap().invert());
    ost.set_parameters(&encoder);
    octx.set_metadata(input_stream.metadata().to_owned());
    format::context::output::dump(&octx, 0, Some(OUTPUT_FILE_NAME));
    //octx.write_header()?;
    std::thread::sleep(std::time::Duration::from_secs(20));
    Ok(())

}
