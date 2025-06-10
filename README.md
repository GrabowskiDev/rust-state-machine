# rust-state-machine

Aplikacja do wizualnego tworzenia i testowania automatów stanów (DAS oraz ε-NAS) z graficznym interfejsem użytkownika opartym o `egui` (biblioteka `eframe`).

## Funkcje

- Tworzenie automatów deterministycznych (DAS) oraz niedeterministycznych z epsilon-przejściami (ε-NAS)
- Edycja alfabetu, stanów, przejść oraz stanów akceptujących przez interfejs graficzny
- Walidacja poprawności automatu (spójność przejść, kompletność, poprawność alfabetu)
- Testowanie ciągów wejściowych i sprawdzanie, czy są akceptowane przez automat
- Obsługa przejść epsilon w trybie ε-NAS

## Uruchomienie

1. Zainstaluj [Rust](https://www.rust-lang.org/tools/install).
2. Sklonuj repozytorium:
```bash
   git clone https://github.com/GrabowskiDev/rust-state-machine.git 
   cd rust-state-machine
```
3. Uruchom aplikację:
```bash
   cargo run
   ```
## Użycie

- Wybierz typ automatu (DAS lub ε-NAS) na górze okna.
- Dodawaj/Usuwaj znaki alfabetu i stany za pomocą przycisków.
- Wypełnij tabelę przejść, wpisując nazwy stanów docelowych (dla ε-NAS można podać kilka stanów oddzielonych przecinkami).
- Zaznacz stany akceptujące.
- Wprowadź ciąg wejściowy i kliknij "Sprawdź", aby zobaczyć wynik.
- Komunikaty o błędach i walidacji pojawią się pod tabelą.

## Struktura projektu

- `src/main.rs` – logika GUI i obsługa interakcji użytkownika
- `src/elements/` – definicje podstawowych struktur: alfabet, stan (Node)
- `src/automats/` – implementacje automatów DAS i ENAS

## Wymagania

- Rust 1.76+ (edycja 2024)
- Biblioteka `eframe` (egui)