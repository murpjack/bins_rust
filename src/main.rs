//extern crate neovim_lib;
//use neovim_lib::{Neovim, NeovimApi, Session};
mod bins;
use bins::BinDay;

fn main() {
    println!("{}", BinDay::print());
    //    let mut event_handler = EventHandler::new();
    //    event_handler.recv();
}

//enum Messages {
//    Show,
//    Unknown(String),
//}
//
//impl From<String> for Messages {
//    fn from(event: String) -> Self {
//        match &event[..] {
//            "show" => Messages::Show,
//            _ => Messages::Unknown(event),
//        }
//    }
//}
//
//struct EventHandler {
//    nvim: Neovim,
//    bin_day: String,
//}
//
//impl EventHandler {
//    fn new() -> EventHandler {
//        let session = Session::new_parent().unwrap();
//        let nvim = Neovim::new(session);
//        let bin_day = BinDay::print();
//
//        EventHandler { nvim, bin_day }
//    }
//
//    fn recv(&mut self) {
//        let receiver = self.nvim.session.start_event_loop_channel();
//
//        for (event, _values) in receiver {
//            match Messages::from(event) {
//                Messages::Show => {
//                    let print = &self.bin_day;
//                    self.nvim.command(&format!("echo \"{}\"", print)).unwrap();
//                }
//
//                // Handle anything else
//                Messages::Unknown(event) => {
//                    self.nvim
//                        .command(&format!("echo \"Unknown command: {}\"", event))
//                        .unwrap();
//                }
//            }
//        }
//    }
//}
