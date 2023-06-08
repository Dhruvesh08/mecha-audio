use gstreamer::prelude::*;
use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    // Build the gsreamer 
    gstreamer::init().unwrap();

    // Get the file path from user input
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    // Build the pipeline
    let pipeline = gstreamer::parse_launch(&format!("playbin uri=file://{file_path}")).unwrap();
    // let pipeline = gstreamer::parse_launch(&format!(
    //     "playbin uri=file://{file_path} ! mpegaudioparse ! mpg123audiodec ! alsasink",
    //     file_path = file_path
    // ))
    // .unwrap();

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
