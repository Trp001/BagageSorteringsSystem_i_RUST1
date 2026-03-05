//      Bufferhåndtering
//   •	Implementer buffere som vektorer (f.eks. Vec<T>) indkapslet i Arc<Mutex<T>> for at sikre trådsikker adgang.
//   •	Buffere skal have en fast kapacitet for at simulere begrænsede ressourcer.
//   4. Simulering af åbning og lukning af terminaler og skranker/checkin
//   •	Terminaler/gate og skranker/checkin skal kunne åbnes og lukkes via brugerinput eller som en del af trådhåndteringen.
//   5. Logning og udskrift
//   •	Brug println! til udskrivning til skærmen.
//   •	Log til filer med std::fs::File og write! makroen for at sikre persistens.
//   6. Monitor og synkronisering
//   •	Implementer monitorsystemer ved hjælp af Arc<Mutex<T>> for at synkronisere adgangen til delt data mellem tråde.
//   Måling (throughput/synkronisering)
//   Indikator: throughput (enheder/sek) og/eller ventetid pr. bufferoperation.
//   Før: Mål baseline på defineret workload.
//   Ændring: Optimer bufferkapaciteter, låsevalg (Mutex/RwLock) og trådefordeling.


//Checkin ved skranken i lufthavnen
pub mod check_in_counter {
 use std::sync::{Arc, Mutex};
 use std::thread;
 use std::thread::JoinHandle;

 pub fn check_in_counter() {
  let _checkin = Arc::new(Mutex::new(()));
  pub struct CheckInCounter {
   pub id: u32,
   baggage_log: i32,
   terminal_log: i32,
   gate_log: i32,
  }
  impl CheckInCounter {
   fn new(id: u32, <>: (/check_in_counter/)) -> CheckInCounter {
    let thread =
        thread::spawn(move ||
            {
             loop {
              let check_in_counter =
                  receiver.lock().unwrap().recv().unwrap();
              {
               ::std::io::_print(format_args!("baggage registered {0} ; executing.\n",
                                              id));
              };
             }
            });
     { id; thread }
   }
  }
 }
}





