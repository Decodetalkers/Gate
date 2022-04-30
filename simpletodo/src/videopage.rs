use std::rc::Rc;

use gtk::prelude::*;
use gtk::Video;
//use gtk::Box;
use gtk4 as gtk;
//use gtk::Label;
use steinsgate::gatewidgetpatterns::*;
pub fn videopage<W>(window: Rc<W>) -> Rc<gtk::Box>
where
    W: IsA<gtk::Window>,
{
    let myvideo = Video::builder().autoplay(true).loop_(false).build();
    let myvideo = Rc::new(myvideo);
    let videoselectrc = myvideo.clone();
    let filechooser = gtk::FileChooserNative::builder()
        .title("Open file")
        .action(gtk::FileChooserAction::Open)
        .accept_label("Open")
        .cancel_label("Cancel")
        .build();
    {
        let videos_filter = gtk::FileFilter::new();
        videos_filter.add_mime_type("video/*");
        videos_filter.set_name(Some("Video"));
        filechooser.add_filter(&videos_filter);

        let audio_filter = gtk::FileFilter::new();
        audio_filter.add_mime_type("audio/*");
        audio_filter.set_name(Some("Audio"));
        filechooser.add_filter(&audio_filter);
    }
    let toplabel = GateLabelPattern {
        text: "select video",
        ..Default::default()
    }
    .prebuild()
    .build();
    let filebutton = GateButtonPattern {
        text: "select",
        ..Default::default()
    }
    .prebuild()
    .build()
    .set_onclick(move |_| {
        let video = videoselectrc.clone();
        filechooser.set_transient_for(Some(&*window));
        filechooser.connect_response(move |d, responce| {
            if responce == gtk::ResponseType::Accept {
                video.set_file(Some(&d.file().unwrap()));
            }
            d.destroy();
        });
        filechooser.show();
    });
    let topbar = GateBoxPattern {
        orientation: gtk::Orientation::Horizontal,
        margin_bottom: 15,
        ..Default::default()
    }
    .prebuild()
    .build();
    topbar.append(&toplabel);
    topbar.append(&filebutton);
    let contailer = GateBoxPattern {
        halign: gtk4::Align::Fill,
        valign: gtk4::Align::Fill,
        margin_end: 15,
        margin_top: 15,
        margin_start: 15,
        margin_bottom: 15,
        ..Default::default()
    }
    .prebuild()
    .build();
    contailer.append(&topbar);
    contailer.append(&*myvideo);
    Rc::new(contailer)
}
