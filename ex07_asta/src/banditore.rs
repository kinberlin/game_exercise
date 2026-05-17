use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use crate::types::{MsgBanditore, MsgPartecipante, RisultatoAsta};

/// Logica del thread banditore.
///
/// I canali verso i partecipanti vengono passati già pronti.
/// Flusso:
/// 1. Invia descrizione prodotto + prezzo minimo a tutti i partecipanti.
/// 2. Attende il messaggio Unisciti da ciascun partecipante.
/// 3. Ciclo d'asta: raccoglie offerte/ritiri, alza il prezzo, finché tutti si ritirano.
/// 4. Scrive il risultato nella struttura condivisa e notifica il vincitore.
pub fn esegui_banditore_con_canali(
    prodotto: String,
    prezzo_iniziale: u32,
    prezzo_riserva: u32,
    incremento: u32,
    rx: Receiver<(String, MsgPartecipante)>,
    canali: HashMap<String, Sender<MsgBanditore>>,
    risultato: Arc<Mutex<Option<RisultatoAsta>>>,
) {
    let n = canali.len();

    // Fase 1: invia la descrizione del prodotto a tutti i partecipanti.
    let mut prezzo_corrente = prezzo_iniziale;
    for tx in canali.values() {
        let _ = tx.send(MsgBanditore::Prodotto {
            nome: prodotto.clone(),
            prezzo_min: prezzo_corrente,
        });
    }

    // Fase 2: attende che tutti i partecipanti si uniscano (conferma di ricezione).
    for _ in 0..n {
        if let Ok((nome, MsgPartecipante::Unisciti)) = rx.recv() {
            println!("[Banditore] {} si è unito all'asta.", nome);
        }
    }

    // Fase 3: ciclo d'asta.
    let mut attivi: Vec<String> = canali.keys().cloned().collect();
    let mut miglior_offerente: Option<String> = None;
    let mut miglior_prezzo = 0u32;

    while !attivi.is_empty() {
        let mut nuovi_ritirati: Vec<String> = vec![];
        let mut offerte_round: Vec<(String, u32)> = vec![];

        for _ in 0..attivi.len() {
            if let Ok((nome, msg)) = rx.recv() {
                match msg {
                    MsgPartecipante::Offerta(importo) => {
                        println!("[Banditore] {} offre {}.", nome, importo);
                        offerte_round.push((nome, importo));
                    }
                    MsgPartecipante::Ritiro => {
                        println!("[Banditore] {} si ritira.", nome);
                        nuovi_ritirati.push(nome);
                    }
                    _ => {}
                }
            }
        }

        // Rimuove i ritirati e li notifica.
        for nome in &nuovi_ritirati {
            attivi.retain(|n| n != nome);
            if let Some(tx) = canali.get(nome) {
                let _ = tx.send(MsgBanditore::Fine);
            }
        }

        if offerte_round.is_empty() {
            break;
        }

        // Seleziona l'offerta più alta del round.
        let (nome_vincitore, importo) = offerte_round
            .into_iter()
            .max_by_key(|(_, v)| *v)
            .unwrap();

        if importo > miglior_prezzo {
            miglior_prezzo = importo;
            miglior_offerente = Some(nome_vincitore);
        }

        // Alza il prezzo e informa i partecipanti ancora attivi.
        prezzo_corrente = miglior_prezzo + incremento;
        for nome in &attivi {
            if let Some(tx) = canali.get(nome) {
                let _ = tx.send(MsgBanditore::NuovoPrezzo(prezzo_corrente));
            }
        }
    }

    // Fase 4: determina l'esito e notifica.
    let esito = if miglior_prezzo >= prezzo_riserva {
        if let Some(ref vincitore) = miglior_offerente {
            println!(
                "[Banditore] Asta vinta da {} a {} per '{}'.",
                vincitore, miglior_prezzo, prodotto
            );
            if let Some(tx) = canali.get(vincitore) {
                let _ = tx.send(MsgBanditore::Vittoria {
                    prodotto: prodotto.clone(),
                    prezzo: miglior_prezzo,
                });
            }
        }
        RisultatoAsta {
            prodotto: prodotto.clone(),
            prezzo: miglior_prezzo,
            vincitore: miglior_offerente.clone(),
        }
    } else {
        println!(
            "[Banditore] Asta fallita: prezzo {} non supera la riserva {}.",
            miglior_prezzo, prezzo_riserva
        );
        RisultatoAsta {
            prodotto: prodotto.clone(),
            prezzo: miglior_prezzo,
            vincitore: None,
        }
    };

    // Invia Fine a chi non ha ancora ricevuto il messaggio.
    for nome in &attivi {
        if miglior_offerente.as_deref() != Some(nome) {
            if let Some(tx) = canali.get(nome) {
                let _ = tx.send(MsgBanditore::Fine);
            }
        }
    }

    *risultato.lock().unwrap() = Some(esito);
}
