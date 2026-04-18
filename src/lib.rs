//! # Game Exercise Library
//! 
//! A collection of programming exercises for learning Rust.
//! Each exercise is organized as a module for easy testing and reuse.

pub mod game_exercise;

use std::collections::HashMap;
// ─── DistributoreAutomatico ───────────────────────────────────────────────────

pub struct DistributoreAutomatico {
    pub prodotti: HashMap<String, i32>,
    pub prodotto_in_erogazione: Option<String>,
    // Option permette di estrarre (take) lo stato prima della chiamata,
    // poi riassegnare il valore di ritorno — risolve il borrow checker senza placeholder.
    stato: Option<Box<dyn StatoDistributore>>,
}

impl DistributoreAutomatico {
    pub fn new(prodotti: HashMap<String, i32>) -> Self {
        DistributoreAutomatico {
            prodotti,
            prodotto_in_erogazione: None,
            stato: Some(Box::new(InAttesaCarta)),
        }
    }

    /// Solo per i test: bypassa il lancio moneta e forza CartaAccettata.
    pub fn forza_carta_accettata(&mut self) {
        self.stato = Some(Box::new(CartaAccettata));
    }

    fn dispatch<F>(&mut self, f: F)
    where
        F: FnOnce(Box<dyn StatoDistributore>, &mut DistributoreAutomatico) -> Box<dyn StatoDistributore>,
    {
        let stato = self.stato.take().expect("stato mancante");
        let nuovo = f(stato, self);
        self.stato = Some(nuovo);
    }

    pub fn inserisci_carta(&mut self) {
        self.dispatch(|s, d| s.inserisci_carta(d));
    }

    pub fn seleziona_prodotto(&mut self, prodotto: &str) {
        self.dispatch(|s, d| s.seleziona_prodotto(d, prodotto));
    }

    pub fn conferma_selezionato(&mut self) {
        self.dispatch(|s, d| s.conferma_selezionato(d));
    }

    pub fn cancella(&mut self) {
        self.dispatch(|s, d| s.cancella(d));
    }

    pub fn preleva_prodotto(&mut self) {
        self.dispatch(|s, d| s.preleva_prodotto(d));
    }
}

// ─── Lancio moneta ───────────────────────────────────────────────────────────

fn lancio_moneta() -> bool {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos();
    nanos % 2 == 0
}