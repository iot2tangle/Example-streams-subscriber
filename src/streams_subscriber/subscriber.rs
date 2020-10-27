//!
//! Channel Subscriber
//!
use crate::streams_subscriber::random_seed;

use iota_streams::app::transport::tangle::{
    client::Client,
    PAYLOAD_BYTES,
};
use iota_streams::app_channels::api::tangle::{Address, Subscriber};

use base64::{decode_config, URL_SAFE_NO_PAD};

use anyhow::Result;

///
/// Channel subscriber
///
pub struct Channel {
    subscriber: Subscriber<Client>,
    announcement_link: Address,
    subscription_link: Address,
    channel_address: String,
}

impl Channel {
    ///
    /// Initialize the subscriber
    ///
    pub fn new(
        node: String,
        channel_address: String,
        announcement_tag: String,
        seed_option: Option<String>,
    ) -> Channel {
        let seed = match seed_option {
            Some(seed) => seed,
            None => random_seed(),
        };
        let client: Client = Client::new_from_url(&node);
        let subscriber = Subscriber::new(&seed, "utf-8", PAYLOAD_BYTES, client);

        Self {
            subscriber: subscriber,
            announcement_link: Address::from_str(&channel_address, &announcement_tag).unwrap(),
            subscription_link: Address::default(),
            channel_address: channel_address,
        }
    }

    ///
    /// Connect
    ///
    pub fn connect(&mut self) -> Result<String> {
        self.subscriber
            .receive_announcement(&self.announcement_link)?;
        Ok(self.subscription_link.msgid.to_string())
    }
    ///
    /// Read signed packet
    ///
    pub fn read_signed(
        &mut self,
        signed_packet_tag: String,
    ) -> Result<Vec<(Option<String>, Option<String>)>> {
        let mut response: Vec<(Option<String>, Option<String>)> = Vec::new();

        let link = Address::from_str(&self.channel_address, &signed_packet_tag).unwrap();

        let (_, public_payload, _) = self.subscriber.receive_signed_packet(&link)?;
        response.push((
            unwrap_data(&String::from_utf8(public_payload.0).unwrap()).unwrap(),
            None, //Iot2Tanagle currently only support public masseges
        ));

        Ok(response)
    }

    ///
    /// Generates the next message in the channels
    ///
    pub fn get_next_message(&mut self) -> Option<Vec<String>> {
        let mut ids: Vec<String> = vec![];

        let mut msgs = self.subscriber.fetch_next_msgs();

        for msg in &msgs {
            ids.push(msg.link.msgid.to_string());
        }

        while !msgs.is_empty() {
            msgs = self.subscriber.fetch_next_msgs();

            for msg in &msgs {
                ids.push(msg.link.msgid.to_string());
            }
        }

        Some(ids)
    }
}

pub fn unwrap_data(data: &str) -> failure::Fallible<Option<String>> {
    let data_str = data.to_string();
    if data_str.len() == 0 {
        return Ok(None);
    }
    let raw = &data.to_string();
    let decode_data = decode_config(&raw, URL_SAFE_NO_PAD)?;
    Ok(Some(String::from_utf8(decode_data).unwrap()))
}
