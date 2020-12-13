use local::streams_subscriber::subscriber::Channel;
use serde_json::Result;
use std::fs::File;
use std::{env, thread, time};

pub struct Subscriber {
    channel_subscriber: Channel,
}

impl Subscriber {
    pub async fn new(node: String, channel_address: String, seed: Option<String>) -> Self {
        let str_iter = channel_address.split(":").collect::<Vec<&str>>();
        let address = str_iter[0];
        let msg_id = str_iter[1];
        let subscriber: Channel = Channel::new(node, address.to_string(), msg_id.to_string(), seed);
        Self {
            channel_subscriber: subscriber,
        }
    }

    ///
    /// Derives Msg Ids for channel and reads messages associated with them,
    /// returns an empty vector if no now messages where found
    ///
    fn read_all_public(&mut self) -> Result<Vec<String>> {
        let tag_list = self.channel_subscriber.get_next_message().unwrap();

        let mut msg_list: Vec<String> = vec![];
        for signed_message_tag in tag_list {
            let msgs: Vec<(Option<String>, Option<String>)> = self
                .channel_subscriber
                .read_signed(signed_message_tag)
                .unwrap();
            for (msg_p, _msg_m) in msgs {
                match msg_p {
                    None => continue,
                    Some(message) => msg_list.push(message),
                }
            }
        }

        Ok(msg_list)
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let channel_address = &args[1];

    let config: serde_json::Value =
        serde_json::from_reader(File::open("config.json").unwrap()).unwrap();

    let node = config["node"].as_str().unwrap().to_string();

    let mut sub = Subscriber::new(node, channel_address.to_string(), None).await;

    sub.channel_subscriber.connect().unwrap();
    println!("Connection to channel established successfully! \n Reading messages...");

    // read old messages in channel
    let public_list = sub.read_all_public().unwrap();
    for data in &public_list {
        //print out a pretty pretty JSON
        println!("{} \n  \n", &data.replace("\\", ""));
    }
    println!("Read all historic Messages! \n Reading New Messages...");

    // listen for new messages sent to channel
    let mut public_list_len: usize = public_list.len();
    loop {
        let public_list = sub.read_all_public().unwrap();

        if &public_list.len() != &public_list_len.clone() {
            match public_list.last() {
                Some(last_data) => {
                    //print out a pretty pretty JSON
                    println!("{} \n  \n", &last_data.replace("\\", ""));
                }
                None => (),
            }
        }
        public_list_len = public_list.len().clone();
        // dont spam thee node with requests!
        thread::sleep(time::Duration::from_secs(5));
    }
}
