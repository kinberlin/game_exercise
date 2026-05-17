/// Messaggio inviato dai partecipanti al banditore (taggato con il nome del mittente).
#[derive(Debug)]
pub enum MsgPartecipante {
    Unisciti,
    Offerta(u32),
    Ritiro,
}

/// Messaggio inviato dal banditore ai partecipanti.
#[derive(Debug)]
pub enum MsgBanditore {
    Prodotto { nome: String, prezzo_min: u32 },
    NuovoPrezzo(u32),
    Vittoria { prodotto: String, prezzo: u32 },
    Fine,
}

/// Risultato dell'asta scritto nella struttura condivisa al termine.
#[derive(Debug, Clone)]
pub struct RisultatoAsta {
    pub prodotto: String,
    pub prezzo: u32,
    pub vincitore: Option<String>,
}
