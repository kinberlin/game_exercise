/// Test di integrazione per il protocollo di asta inglese.
///
/// Poiché banditore e partecipanti sono thread che comunicano via canali,
/// i test eseguono l'asta completa e verificano il risultato nella struttura condivisa.

// I moduli interni non sono pub nel binario, quindi duplichiamo il minimo necessario
// oppure eseguiamo l'asta direttamente in-process per i test.
//
// Strategia: richiamiamo le funzioni del crate come libreria interna tramite
// un modulo helper inline che rispecchia la logica di main.rs.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;

// Reimportiamo i tipi e le funzioni direttamente dai moduli sorgente.
// Poiché ex07_asta è un crate binario (non lib), includiamo i file via path.
#[path = "../src/types.rs"]
mod types;
#[path = "../src/banditore.rs"]
mod banditore;
#[path = "../src/partecipante.rs"]
mod partecipante;

use types::RisultatoAsta;

/// Esegue un'asta con la configurazione specificata e restituisce il risultato.
fn esegui_asta(
    prodotto: &str,
    prezzo_iniziale: u32,
    prezzo_riserva: u32,
    incremento: u32,
    partecipanti: Vec<(&str, u32)>, // (nome, max_bid)
) -> Option<RisultatoAsta> {
    let (tx_verso_banditore, rx_banditore) =
        mpsc::channel::<(String, types::MsgPartecipante)>();

    let mut canali_verso_partecipanti: HashMap<String, mpsc::Sender<types::MsgBanditore>> =
        HashMap::new();
    let mut rx_partecipanti: Vec<(String, u32, mpsc::Receiver<types::MsgBanditore>)> = vec![];

    for (nome, max_bid) in &partecipanti {
        let (tx, rx) = mpsc::channel::<types::MsgBanditore>();
        canali_verso_partecipanti.insert(nome.to_string(), tx);
        rx_partecipanti.push((nome.to_string(), *max_bid, rx));
    }

    let risultato: Arc<Mutex<Option<RisultatoAsta>>> = Arc::new(Mutex::new(None));
    let risultato_banditore = Arc::clone(&risultato);

    let mut handles = vec![];
    for (nome, max_bid, rx) in rx_partecipanti {
        let tx = tx_verso_banditore.clone();
        let h = thread::spawn(move || {
            partecipante::esegui_partecipante(nome, max_bid, tx, rx);
        });
        handles.push(h);
    }

    let prodotto_str = prodotto.to_string();
    let h_banditore = thread::spawn(move || {
        banditore::esegui_banditore_con_canali(
            prodotto_str,
            prezzo_iniziale,
            prezzo_riserva,
            incremento,
            rx_banditore,
            canali_verso_partecipanti,
            risultato_banditore,
        );
    });

    h_banditore.join().unwrap();
    for h in handles {
        h.join().unwrap();
    }

    let esito = risultato.lock().unwrap().clone();
    esito
}

// ── Vincitore corretto ────────────────────────────────────────────────────────

#[test]
fn test_vince_offerente_con_max_bid_piu_alto() {
    // Alice (max 200) batte Bob (max 170) e Carlo (max 140).
    let r = esegui_asta(
        "Dipinto",
        100,
        150,
        10,
        vec![("Alice", 200), ("Bob", 170), ("Carlo", 140)],
    )
    .expect("l'asta deve produrre un risultato");

    assert_eq!(r.vincitore.as_deref(), Some("Alice"));
    assert!(r.prezzo >= 150, "il prezzo deve superare la riserva");
}

// ── Prezzo di riserva non raggiunto ──────────────────────────────────────────

#[test]
fn test_nessun_vincitore_sotto_riserva() {
    // Tutti hanno max_bid < prezzo_riserva (500).
    let r = esegui_asta(
        "Opera Rara",
        100,
        500,
        10,
        vec![("Alice", 150), ("Bob", 130)],
    )
    .expect("l'asta deve produrre un risultato");

    assert!(r.vincitore.is_none(), "nessuno deve vincere sotto la riserva");
}

// ── Un solo partecipante ──────────────────────────────────────────────────────

#[test]
fn test_un_solo_partecipante_vince() {
    let r = esegui_asta("Vaso", 50, 50, 5, vec![("Solo", 200)])
        .expect("l'asta deve produrre un risultato");

    assert_eq!(r.vincitore.as_deref(), Some("Solo"));
}

// ── Tutti si ritirano subito ──────────────────────────────────────────────────

#[test]
fn test_tutti_si_ritirano() {
    // max_bid di tutti è inferiore al prezzo iniziale.
    let r = esegui_asta("Quadro", 200, 200, 10, vec![("Alice", 100), ("Bob", 50)])
        .expect("l'asta deve produrre un risultato");

    assert!(r.vincitore.is_none());
    assert_eq!(r.prezzo, 0, "nessuna offerta valida, prezzo finale 0");
}

// ── Prodotto corretto nel risultato ──────────────────────────────────────────

#[test]
fn test_prodotto_nel_risultato() {
    let r = esegui_asta("Libro Antico", 10, 10, 5, vec![("Alice", 100)])
        .expect("l'asta deve produrre un risultato");

    assert_eq!(r.prodotto, "Libro Antico");
}
