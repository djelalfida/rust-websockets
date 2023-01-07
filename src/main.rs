use ws::{listen, CloseCode, Error, Handler, Handshake, Message, Result, Sender};

struct Server {
    out: Sender,
}

impl Handler for Server {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        // We have a new connection, so we send a message to the client to let them know
        self.out.send("Hello, client!")
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        // Print the message we received from the client
        println!("Message from client: {}", msg);

        // Send a message back to the client
        self.out.send(msg)
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away => println!("The client is leaving the site."),
            _ => println!("The client encountered an error: {}", reason),
        }
    }

    fn on_error(&mut self, err: Error) {
        println!("The server encountered an error: {:?}", err);
    }
}

fn main() {
    listen("127.0.0.1:5000", |out| Server { out }).unwrap();
}
