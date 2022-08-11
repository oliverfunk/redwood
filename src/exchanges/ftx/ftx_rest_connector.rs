use super::FtxApiDetails;

pub struct FtxRestConnector {
    api_details: Option<FtxApiDetails>,
}

impl FtxRestConnector {
    pub fn new(api_details: Option<FtxApiDetails>) -> Self {
        FtxRestConnector { api_details }
    }
}
