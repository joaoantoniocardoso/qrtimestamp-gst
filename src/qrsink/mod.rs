use gst::glib;
use gst::prelude::*;

mod imp;

glib::wrapper! {
    pub struct QRTimeStampSink(ObjectSubclass<imp::QRTimeStampSink>) @extends gst_base::PushSrc, gst_base::BaseSrc, gst::Element, gst::Object;
}

pub fn register(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    gst::Element::register(
        Some(plugin),
        "qrtimestampsink",
        gst::Rank::NONE,
        QRTimeStampSink::static_type(),
    )
}
