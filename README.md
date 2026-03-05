
## Ændringslog

Tilføjet treads. Arc<Mutex<Vec<String>>>,// vector er buffer. Sikre trådsikker adgang.

- Tilføjet startkode og variabler (Trin 0)
- Implementeret overskrivning af fil (Trin 1)
- Implementeret læsning af fil (Trin 2)
- Implementeret append‑funktion (Trin 3)
- Implementeret tømning af fil (Trin 4)
- Implementeret bonus: slet fil
- Tilføjet test
- Udvidet README med refleksioner og brugervejledning



##  Målenote før/efter

| Før opgaven | Efter opgaven |
|-------------|---------------|
| Jeg kendte kun lidt til filhåndtering i Rust. | Jeg kan nu oprette, læse, tilføje, tømme og slette filer. |
| Jeg brugte ofte `unwrap` og `expect`. | Jeg bruger nu `Result`, `?` og match‑baseret fejlhåndtering. |
| Jeg havde erfaring med systemprogrammering. | Jeg har fået en god introduktion til praktisk systemprogrammering i Rust. |
