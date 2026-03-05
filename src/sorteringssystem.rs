
//Krav til implementering i Rust
//   Implementér et program i Rust, der simulerer bagagesorteringssystemet ved hjælp af tråde og synkronisering.
//   1. Dataoprettelse
//   •	Opret data fra reservationssystemet som statiske data indlejret i datastrukturer eller læs dem fra en tekstfil ved hjælp af std::fs.
//   •	Flyveplanen skal kunne opdateres over tid enten via brugerinput eller tråde.
//   2. Trådhåndtering
//   •	Brug std::thread til at oprette tråde, der repræsenterer skranker, sorteringsanlæg og terminaler.
//   •	Synkronisering mellem tråde skal sikres med std::sync::Mutex eller std::sync::RwLock.

#![feature(prelude_import)]
#[macro_use]

use std::prelude::v1::*;
use std::prelude::rust_2024::*;
pub mod sorteringssystem {
    use core::panicking;
use std::sync::{Arc, Mutex};
    pub fn sorteringssystem() {
        struct Sorteringssystem {
            id: u32,
            baggage_id: Arc<Mutex<Vec<String>>>,
        }
        impl Sorteringssystem {
            pub(crate) fn process_sorteringssystem(&self, p0: &str) {
                std::core::panicking::panic("not yet implemented")
            }
        }
    }
}

