mod types;
mod banditore;
mod partecipante;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;
use banditore::esegui_banditore_con_canali;
use partecipante::esegui_partecipante;
use types::RisultatoAsta;

fn main() {
    let prodotto = "Quadro di Van Gogh".to_string();
    let prezzo_iniziale = 100u32;
    let prezzo_riserva = 150u32;
    let incremento = 10u32;

    // Partecipanti: (nome, offerta_massima)
    let partecipanti_config = vec![
        ("Alice".to_string(), 200u32),
        ("Bob".to_string(), 170u32),
        ("Carlo".to_string(), 140u32),
    ];

    // Canale condiviso: tutti i partecipanti → banditore
    let (tx_verso_banditore, rx_banditore) = mpsc::channel::<(String, types::MsgPartecipante)>();

    // Un canale per ogni partecipante: banditore → partecipante
    let mut canali_verso_partecipanti: HashMap<String, mpsc::Sender<types::MsgBanditore>> = HashMap::new();
    let mut rx_partecipanti: Vec<(String, u32, mpsc::Receiver<types::MsgBanditore>)> = vec![];

    for (nome, max_bid) in &partecipanti_config {
        let (tx, rx) = mpsc::channel::<types::MsgBanditore>();
        canali_verso_partecipanti.insert(nome.clone(), tx);
        rx_partecipanti.push((nome.clone(), *max_bid, rx));
    }

    // Struttura condivisa per il risultato finale
    let risultato: Arc<Mutex<Option<RisultatoAsta>>> = Arc::new(Mutex::new(None));
    let risultato_banditore = Arc::clone(&risultato);

    println!("=== ASTA INGLESE ===");
    println!("Prodotto: {}", prodotto);
    println!("Prezzo iniziale: {}", prezzo_iniziale);
    println!("Prezzo di riserva: {}", prezzo_riserva);
    println!("Incremento: {}", incremento);
    println!("====================\n");

    // Spawna i thread partecipanti.
    let mut handles = vec![];
    for (nome, max_bid, rx) in rx_partecipanti {
        let tx = tx_verso_banditore.clone();
        let h = thread::spawn(move || {
            esegui_partecipante(nome, max_bid, tx, rx);
        });
        handles.push(h);
    }

    // Spawna il thread banditore.
    let h_banditore = thread::spawn(move || {
        esegui_banditore_con_canali(
            prodotto,
            prezzo_iniziale,
            prezzo_riserva,
            incremento,
            rx_banditore,
            canali_verso_partecipanti,
            risultato_banditore,
        );
    });

    // Attende la fine di tutti i thread.
    h_banditore.join().unwrap();
    for h in handles {
        h.join().unwrap();
    }

    // Legge il risultato dalla struttura condivisa.
    println!("\n=== RISULTATO FINALE ===");
    let guard = risultato.lock().unwrap();
    match guard.as_ref() {
        Some(r) => {
            println!("Prodotto: {}", r.prodotto);
            println!("Prezzo finale: {}", r.prezzo);
            match &r.vincitore {
                Some(v) => println!("Vincitore: {}", v),
                None => println!("Nessun vincitore (prezzo di riserva non raggiunto)"),
            }
        }
        None => println!("Nessun risultato disponibile."),
    }
}
