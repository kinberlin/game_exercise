use std::sync::mpsc::{Receiver, Sender};
use crate::types::{MsgBanditore, MsgPartecipante};

/// Logica del thread partecipante.
///
/// Il partecipante ha un `max_bid`: offre finché il prezzo corrente è ≤ max_bid,
/// dopodiché si ritira. Strategia deterministica, utile anche nei test.
pub fn esegui_partecipante(
    nome: String,
    max_bid: u32,
    tx: Sender<(String, MsgPartecipante)>,
    rx: Receiver<MsgBanditore>,
) {
    // Segnala la propria partecipazione.
    let _ = tx.send((nome.clone(), MsgPartecipante::Unisciti));

    loop {
        match rx.recv() {
            Ok(MsgBanditore::Prodotto { nome: prodotto, prezzo_min }) => {
                println!(
                    "[{}] Ricevuto prodotto '{}', prezzo minimo: {}.",
                    nome, prodotto, prezzo_min
                );
                rispondi(&nome, prezzo_min, max_bid, &tx);
            }
            Ok(MsgBanditore::NuovoPrezzo(prezzo)) => {
                println!("[{}] Nuovo prezzo: {}.", nome, prezzo);
                rispondi(&nome, prezzo, max_bid, &tx);
            }
            Ok(MsgBanditore::Vittoria { prodotto, prezzo }) => {
                println!(
                    "[{}] HAI VINTO '{}' a {}!",
                    nome, prodotto, prezzo
                );
                break;
            }
            Ok(MsgBanditore::Fine) | Err(_) => {
                println!("[{}] Asta terminata.", nome);
                break;
            }
        }
    }
}

fn rispondi(
    nome: &str,
    prezzo: u32,
    max_bid: u32,
    tx: &Sender<(String, MsgPartecipante)>,
) {
    if prezzo <= max_bid {
        println!("[{}] Offro {}.", nome, prezzo);
        let _ = tx.send((nome.to_string(), MsgPartecipante::Offerta(prezzo)));
    } else {
        println!("[{}] Mi ritiro (max_bid={}).", nome, max_bid);
        let _ = tx.send((nome.to_string(), MsgPartecipante::Ritiro));
    }
}
