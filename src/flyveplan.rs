//Krav til implementering i Rust
//   Implementér et program i Rust, der simulerer bagagesorteringssystemet ved hjælp af tråde og synkronisering.
//   1. Dataoprettelse
//   •	Opret data fra reservationssystemet som statiske data indlejret i datastrukturer eller læs dem fra en tekstfil ved hjælp af std::fs.
//   •	Flyveplanen skal kunne opdateres over tid enten via brugerinput eller tråde.
//   2. Trådhåndtering
//   •	Brug std::thread til at oprette tråde, der repræsenterer skranker/checkin, sorteringsanlæg og terminaler/gate.
//   •	Synkronisering mellem tråde skal sikres med std::sync::Mutex eller std::sync::RwLock.

// flyveplan/terminal/gate
use std::sync::{Arc, Mutex};
use std::thread;

pub fn flyveplan() {
    struct Terminal {
        id: u32,
        boardingcard_id: Arc<Mutex<Vec<String>>>,
    }
    impl Terminal {
        pub(crate) fn process_boardingcard(&self, p0: &str) {
            todo!()
        }
    }

    impl Terminal {
        fn new(id: u32, _boardingcard_id: Arc<Mutex<Vec<String>>>) -> Self {
            Self { id, boardingcard_id: Arc::new(Mutex::new(vec![])) }
        }

        //noinspection ALL
        const PROCESS_BOARDINGCARD: fn(&str) = |boardingcard_id: &str| {

                //noinspection ALL
                static mut id: _ = self.boardingcard_log.lock().unwrap();
                let x = log.push(format!("Check-in {} registered boarding card: {}", Self { /* fields */ }.id, boardingcard_id));
            println!("terminal {} registered boarding card: {}", self.id, boardingcard_id);
        };
    }

    pub fn main() {
        let boarding_log = Arc::new(Mutex::new(Vec::new()));

        let terminal1 = Terminal::new(1, boarding_log.clone());
        let handle = thread::spawn(move || {
            terminal1.process_boardingcard("Boarding id");
        });

        handle.join().unwrap();
    }
}
