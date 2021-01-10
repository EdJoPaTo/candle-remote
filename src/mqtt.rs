use rumqttc::{Client, Connection, MqttOptions, QoS};
use std::thread;

pub struct Sender {
    base_topic: String,
    client: Client,
    thread_handle: Option<thread::JoinHandle<()>>,
}

impl Sender {
    pub fn new(host: &str, port: u16, base_topic: &str) -> Sender {
        let client_id = format!("candle-remote-{:x}", rand::random::<u32>());
        let mqttoptions = MqttOptions::new(client_id, host, port);
        let (client, connection) = Client::new(mqttoptions, 10);

        let thread_handle = thread::Builder::new()
            .name("mqtt connection".into())
            .spawn(move || thread_logic(connection))
            .expect("failed to start mqtt thread");

        Sender {
            client,
            base_topic: base_topic.to_owned(),
            thread_handle: Some(thread_handle),
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn send<P: ToString>(&mut self, verb: &str, payload: P, retain: bool) {
        let topic = format!("{}/set/{}", &self.base_topic, verb);
        self.client
            .publish(topic, QoS::AtLeastOnce, retain, payload.to_string())
            .expect("failed to send via mqtt");
    }
}

fn thread_logic(mut connection: Connection) {
    for notification in connection.iter() {
        if let rumqttc::Event::Outgoing(rumqttc::Outgoing::Disconnect) =
            notification.expect("mqtt connection error")
        {
            break;
        }
    }
}

impl Drop for Sender {
    fn drop(&mut self) {
        // Try to disconnect and wait but dont care if that doesnt work (-> or default)
        self.client.disconnect().unwrap_or_default();
        if let Some(thread_handle) = self.thread_handle.take() {
            thread_handle.join().unwrap_or_default();
        }
    }
}
