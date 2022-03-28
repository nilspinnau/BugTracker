

// First step: Use a websocket, where the client writes to a server, this server than handles the connection

// implement a simple protocol for the chat, e.g. 
// how do we send the history of the chat? We have to incroporate this in the protocol aswell


// this is all "json", maybe use xml or even html (no translation needed) instead? 
struct WebsocketMessage {
    message: i64,               // 256 characters -> 256*8
    timestamp: i64,
}

enum MessageTYPE {
    END,
    WRITE,
    RECEIVE
}

struct ProtocolMessage {
    message_type: MessageTYPE,           // END, WRITE, RECEIVE
    sender: u64,                    
    receiver: u64,
    count: u64,                          // how long is the history vector
    data: Vec<WebsocketMessage>       // this is then a dataStream
}
fn convert_to_html() {}
fn convert_to_json() {}



// either we give the client a json object, or we already convert it to html 



// 1. user opens chat -> start websocket
// 2. server accepts request -> opens new thread for handling the chat
// 3. user writes message
// 4. server receives and writes to database
// how do we update the web chat interface??? 


// either we handle the chat without database and we require both clients to be open (if we spawn a chat handler for each websocket request this might be huge overhead)

// Better??
// alternatively the client message is saved to a sql database, where we have {sender, receiver, message, timestamp}
// this is then collected, when the receiver opens a chat aswell, for each chat, check sql database therefore

use std::thread;



pub fn run_chat_handler() {


    loop {
        // new request from websocket, start new chat handler
        thread::spawn(||{});
    }
}


fn recv() -> String {
    "hello".to_string()
}
    


fn read_database() {}

fn update_database() {}

fn remove_client_from_active() {}

fn check_other_clients() {}

// the client sends json like data?
fn parse_from_client(message: String) -> ProtocolMessage {
    ProtocolMessage { message_type: MessageTYPE::END, sender: 0, receiver: 0, count: 0, data: Vec::new() }
}

// this handles only the writing client, we have to also check for another client, which might be the receiver and send the message as well
fn handle_client() {

    loop {
        let request = recv();
        let request: ProtocolMessage = parse_from_client(request);
        let data = match request.message_type {
            MessageTYPE::RECEIVE => {
                // this message we get, when we open the chat
                read_database()
            },
            MessageTYPE::WRITE => {
                update_database()
            },
            MessageTYPE::END => {
                remove_client_from_active();
                return
            },  // end the chat here
        };
        // let answer_html_string = convert_to_html(data).to_string();
        // check_other_clients(client_array); 
        // send(answer_html_string); 
    }
}


