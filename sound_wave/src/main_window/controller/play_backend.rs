extern crate gstreamer as gst;
extern crate gstreamer_player as gst_player;
use gst::prelude::*;
use gst::{SeekFlags, SeekType, State, Element};
use gst::event::{Seek, Step};
use std::sync::mpsc;
use gstreamer::ClockTime;

use super::Command;

use single_value_channel::channel_starting_with;

fn format_path(pth: String) -> String{
    let mut s: String = String::new();
    for i in pth.chars(){
        if i == '\\'{
            s.push('/');
        }
        else{
            s.push(i);
        }
    }

    return s;
} 


pub fn start(song: String, rx: mpsc::Receiver<Command>, tx: single_value_channel::Updater<u64>, ltx: single_value_channel::Updater<u64>) {
    // Initialize GStreamer
    gst::init().unwrap();
    // Build the pipeline
    //let uri = format!("file:///{song}");
    let uri = format_path(format!("file:///{song}"));
    let pipeline = gst::parse_launch(&format!("playbin uri={}", uri)).unwrap();

    // Start playing
    pipeline.set_state(gst::State::Playing).expect("Unable to set the pipeline to the `Playing` state");

    let mut playing = true;
    let mut lenght_of_song: Option<u64> = None;

    loop
    {
        match rx.try_recv(){
            Ok(c) => {
                match c
                {
                    Command::PlayPause =>
                        {
                        if playing
                        {
                            pipeline.set_state(State::Paused);
                        }
                        else
                        {
                            pipeline.set_state(State::Playing);
                        }
                        playing = !playing;
                        ()
                    }

                    Command::Forward =>
                        {

                            if let Some(audio_sink) = pipeline.property::<Option<Element>>("audio-sink")
                            {
                                // Send the event
                                println!("v if");
                                let step = Step::new(gst::format::ClockTime::from_seconds(10), 1., true, false);
                                audio_sink.send_event(step);
                                //println!("{:?}", (audio_sink.query_position::<gst::format::ClockTime>().unwrap()));

                            }
                            ()
                    }
                    Command::Back =>
                        {
                            if let Some(audio_sink) = pipeline.property::<Option<Element>>("audio-sink")
                            {
                                //println!("{:?}", (audio_sink.query_position::<gst::format::ClockTime>()));

                                audio_sink.seek_simple(gst::SeekFlags::FLUSH,
                                                 gst::format::ClockTime::from_nseconds(
                                                 audio_sink.query_position::<gst::format::ClockTime>().unwrap().abs_diff(10000000000)
                                                 )).expect("panic message");
                                //println!("{:?}", (audio_sink.query_position::<gst::format::ClockTime>()));
                            }
                            ()
                    }
                    Command::SetPosInSeconds(v) =>
                        {
                            let v = (v * lenght_of_song.unwrap()) / 100;
                            if let Some(audio_sink) = pipeline.property::<Option<Element>>("audio-sink")
                            {
                                //println!("{:?}", (audio_sink.query_position::<gst::format::ClockTime>()));

                                audio_sink.seek_simple(gst::SeekFlags::FLUSH,
                                                       gst::format::ClockTime::from_nseconds(v)).expect("panic message");
                                match pipeline.query_position::<gst::format::ClockTime>(){
                                    Some(t) => {
                                        tx.update(t.abs_diff(0)*100/lenght_of_song.unwrap());
                                        //println!("{:?}", a);
                                        //println!("{:?}", t.abs_diff(0)*100/lenght_of_song.unwrap());
                                        ()
                                    },
                                    None => ()
                                }
                            }
                            ()
                    },

                    Command::Quit => {
                        pipeline
                            .set_state(gst::State::Null)
                            .expect("Unable to set the pipeline to the `Null` state");
                        return;
                    }

                    _ => ()

                }
                ()
                }

            Err(e) => (),
        }
        if lenght_of_song == None{
            match pipeline.query_duration::<gst::format::ClockTime>(){
                Some(t) => {
                    lenght_of_song = Some(t.abs_diff(0));
                    ltx.update(t.seconds());
                    ()
                },
                _ => ()
            }
            //println!("{:?}", lenght_of_song);
        }
        else
        {
            //println!("{:?}", lenght_of_song);
            match pipeline.query_position::<gst::format::ClockTime>(){
                Some(t) => {
                    tx.update(t.abs_diff(0)*100/lenght_of_song.unwrap());
                    //println!("{:?}", a);
                    //println!("{:?}", t.abs_diff(0)*100/lenght_of_song.unwrap());
                    ()
                },
                None => ()
            }
        }
    }

    // Wait until error or EOS
    let bus = pipeline.bus().unwrap();
    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;

        match rx.try_recv(){
            Ok(T) => println!("ok"),
            Err(E) => println!("err\n"),
        }

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
        .set_state(gst::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");
}
