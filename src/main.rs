use gstreamer::prelude::*;

use std::error::Error;

//improve this code as gstreamer use gstreamer::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    gstreamer::init().unwrap();

    // Build the pipeline
    gstreamer::init().unwrap();

    // Build the pipeline
    let file_path = "https://www.soundhelix.com/examples/mp3/SoundHelix-Song-1.mp3";
    let pipeline = gstreamer::parse_launch(&format!(
        "playbin uri={file_path} audio-sink=autoaudiosink",
        file_path = file_path
    ))
    .unwrap();

    // Start playing
    pipeline
        .set_state(gstreamer::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");

    // Wait until error or EOS
    let bus = pipeline.bus().unwrap();
    for msg in bus.iter_timed(gstreamer::ClockTime::NONE) {
        use gstreamer::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.src().map(|s| s.path_string()),
                    err.error(),
                    err.debug()
                );
                break;
            }
            _ => (),
        }
    }

    // Shutdown pipeline
    pipeline
        .set_state(gstreamer::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");


    Ok(())
}
