use std::time::Duration;

use iced::futures::{SinkExt, Stream, channel::mpsc::Sender};
use kuecard_backend::message::Message;

use crate::{
    custommessage::CustomMessage,
    globals::{AD_REFRESH_INTERVAL_SECONDS, AD_URL},
    helpers::AdMetadata,
};

pub fn start_ad_delivery() -> impl Stream<Item = Message<CustomMessage>> {
    return iced::stream::channel(AD_REFRESH_INTERVAL_SECONDS, |mut tx: Sender<Message<CustomMessage>>| async move {
        loop {
            let data: Option<(Message<CustomMessage>, String)> = internal_start_ad_delivery(
                String::from(AD_URL)
            ).await;

            let s = tx.send(data.unwrap().0).await;

            if s.is_err() {
                continue;
            }

            tokio::time::sleep(Duration::from_secs(AD_REFRESH_INTERVAL_SECONDS as u64)).await;
        }
    });
}

async fn internal_start_ad_delivery(url: String) -> Option<(Message<CustomMessage>, String)> {
    let res: Result<AdMetadata, String> = fetch_ad(&url).await;
    let opt: Option<(Message<CustomMessage>, String)> = match res {
        Result::Ok(ad_metadata) => Option::Some((
            Message::Custom(CustomMessage::AdLoaded(Result::Ok(ad_metadata))),
            url,
        )),
        Result::Err(err) => Option::Some((
            Message::Custom(CustomMessage::AdLoaded(Result::Err(err))),
            url,
        )),
    };
    return opt;
}

pub fn is_url_valid(url: &str) -> bool {
    return url.starts_with("http://") || url.starts_with("https://");
}

async fn fetch_ad(url: &str) -> Result<AdMetadata, String> {
    if !is_url_valid(url) {
        return Result::Err(String::from("URL is not valid (requires prefix \"http://\" or \"https://\")"));
    }

    let res = reqwest::get(url).await;

    if res.is_err() {
        return Result::Err(res.err().unwrap().to_string());
    }

    let response = res.unwrap();

    let res = response.json().await;

    if res.is_err() {
        return Result::Err(res.err().unwrap().to_string());
    }

    let mut ad_metadata: AdMetadata = res.ok().unwrap();

    let res = ad_metadata.ad_image.load_image_bytes_from_url().await;

    if res.is_err() {
        return Result::Err(res.err().unwrap());
    }

    return Result::Ok(ad_metadata);
}
