////Krav til implementering i Rust
// //   Implementér et program i Rust, der simulerer bagagesorteringssystemet ved hjælp af tråde og synkronisering.
// //   1. Dataoprettelse
// //   •	Opret data fra reservationssystemet som statiske data indlejret i datastrukturer eller læs dem fra en tekstfil ved hjælp af std::fs.
// //   •	Flyveplanen skal kunne opdateres over tid enten via brugerinput eller tråde.
// //   2. Trådhåndtering
// //   •	Brug std::thread til at oprette tråde, der repræsenterer skranker, sorteringsanlæg og terminaler.
// //   •	Synkronisering mellem tråde skal sikres med std::sync::Mutex eller std::sync::RwLock.



pub mod reservationssystem {
    pub fn reservationssysten() {}
}
pub mod flyveplan {
    use std::sync::{Arc, Mutex};
    use std::thread;
    
    pub fn flyveplan() {
        struct Terminal {
            id: u32,
            boardingcard_id: Arc<Mutex<Vec<String>>>,
        }
        impl Terminal {
            pub(crate) fn process_boardingcard(&self, p0: &str) {
                ::core::panicking::panic("not yet implemented")
            }
        }
        impl Terminal {
            fn new(id: u32, _boardingcard_id: Arc<Mutex<Vec<String>>>)
                   -> Self {
                Self {
                    id,
                    boardingcard_id: Arc::new(Mutex::new(::alloc::vec::Vec::new())),
                }
            }
            const PROCESS_BOARDINGCARD: fn(&str) =
                |boardingcard_id: &str|
                    {
                        static mut id: _ = self.boardingcard_log.lock().unwrap();
                        let x =
                            log.push(::alloc::__export::must_use({
                                ::alloc::fmt::format(format_args!("Check-in {0} registered boarding card: {1}",
                                                                  Self.id, boardingcard_id))
                            }));
                        {
                            ::std::io::_print(format_args!("terminal {0} registered boarding card: {1}\n",
                                                           self.id, boardingcard_id));
                        };
                    };
        }
        fn main() {
            let boarding_log = Arc::new(Mutex::new(Vec::new()));
            let terminal1 = Terminal::new(1, boarding_log.clone());
            let handle =
                thread::spawn(move ||
                    { terminal1.process_boardingcard("Boarding id"); });
            handle.join().unwrap();
        }
    }
}
