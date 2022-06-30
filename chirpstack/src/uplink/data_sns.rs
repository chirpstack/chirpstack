use tracing::{span, Instrument, Level};

use super::{data, UplinkFrameSet};

pub struct Data {}

impl Data {
    pub async fn handle(ufs: UplinkFrameSet) {
        let span = span!(Level::INFO, "data_up_sns");
        data::Data::handle(ufs).instrument(span).await
    }
}
