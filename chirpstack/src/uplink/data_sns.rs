use anyhow::Result;
use tracing::{span, Instrument, Level};

use super::{data, UplinkFrameSet};

pub struct Data {}

impl Data {
    pub async fn handle(ufs: UplinkFrameSet) -> Result<()> {
        let span = span!(Level::INFO, "data_up_sns", dev_eui = tracing::field::Empty);
        data::Data::_handle(ufs).instrument(span).await
    }
}
