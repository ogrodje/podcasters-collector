macro_rules! anchor_episodes_url {
    () => {
        "https://anchor.fm/api/proxy/v3/stations/\
        webStationId:{}/episodePage?limit=50&orderBy=publishOn"
    };
}
pub(crate) use anchor_episodes_url;

pub const ANCHOR_CSRF_URL: &'static str = "https://anchor.fm/api/csrf";
pub const ANCHOR_LOGIN_URL: &'static str = "https://anchor.fm/api/login";
pub const ANCHOR_METADATA_URL: &'static str = "https://anchor.fm/api/podcast/metadata";
