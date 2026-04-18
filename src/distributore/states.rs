

pub trait StatoDistributore {
    fn inserisci_carta(self: Box<Self>, d: &mut DistributoreAutomatico) -> Box<dyn StatoDistributore>;
    fn seleziona_prodotto(self: Box<Self>, d: &mut DistributoreAutomatico, prodotto: &str) -> Box<dyn StatoDistributore>;
    fn conferma_selezionato(self: Box<Self>, d: &mut DistributoreAutomatico) -> Box<dyn StatoDistributore>;
    fn cancella(self: Box<Self>, d: &mut DistributoreAutomatico) -> Box<dyn StatoDistributore>;
    fn preleva_prodotto(self: Box<Self>, d: &mut DistributoreAutomatico) -> Box<dyn StatoDistributore>;
}

// ─── Stati concreti ──────────────────────────────────────────────────────────

pub struct InAttesaCarta;
pub struct CartaAccettata;
pub struct SelezionatoProdotto;
pub struct ProdottoDisponibile;
pub struct ProdottoEsaurito;

// ─── Macro helper per "Operazione non Valida" ────────────────────────────────
macro_rules! non_valida {
    ($self:ident) => {{
        println!("Operazione non Valida");
        $self
    }};
}

// ─── InAttesaCarta ───────────────────────────────────────────────────────────

impl StatoDistributore for InAttesaCarta {
    fn inserisci_carta(self: Box<Self>, d: &mut DistributoreAutomatico) -> Box<dyn StatoDistributore> {
        if lancio_moneta() {
            println!("Carta Accettata");
            Box::new(CartaAccettata)
        } else {
            println!("Carta NON Accettata");
            self
        }
    }
    fn seleziona_prodotto(self: Box<Self>, _: &mut DistributoreAutomatico, _: &str) -> Box<dyn StatoDistributore> { non_valida!(self) }
    fn conferma_selezionato(self: Box<Self>, _: &mut DistributoreAutomatico) -> Box<dyn StatoDistributore> { non_valida!(self) }
    fn cancella(self: Box<Self>, _: &mut DistributoreAutomatico) -> Box<dyn StatoDistributore> { non_valida!(self) }
    fn preleva_prodotto(self: Box<Self>, _: &mut DistributoreAutomatico) -> Box<dyn StatoDistributore> { non_valida!(self) }
}

// ─── CartaAccettata ──────────────────────────────────────────────────────────

impl StatoDistributore for CartaAccettata {
    fn inserisci_carta(self: Box<Self>, _: &mut DistributoreAutomatico) -> Box<dyn StatoDistributore> { non_valida!(self) }
    fn seleziona_prodotto(self: Box<Self>, d: &mut DistributoreAutomatico, prodotto: &str) -> Box<dyn StatoDistributore> {
        if d.prodotti.contains_key(prodotto) {
            println!("Prodotto {} Presente", prodotto);
            d.prodotto_in_erogazione = Some(prodotto.to_string());
            Box::new(SelezionatoProdotto)
        } else {
            println!("Scegli altro Prodotto");
            self
        }
    }
    fn conferma_selezionato(self: Box<Self>, _: &mut DistributoreAutomatico) -> Box<dyn StatoDistributore> { non_valida!(self) }
    fn cancella(self: Box<Self>, d: &mut DistributoreAutomatico) -> Box<dyn StatoDistributore> {
        println!("Operazione Cancellata");
        d.prodotto_in_erogazione = None;
        Box::new(InAttesaCarta)
    }
    fn preleva_prodotto(self: Box<Self>, _: &mut DistributoreAutomatico) -> Box<dyn StatoDistributore> { non_valida!(self) }
}

// ─── SelezionatoProdotto ─────────────────────────────────────────────────────

impl StatoDistributore for SelezionatoProdotto {
    fn inserisci_carta(self: Box<Self>, _: &mut DistributoreAutomatico) -> Box<dyn StatoDistributore> { non_valida!(self) }
    fn seleziona_prodotto(self: Box<Self>, _: &mut DistributoreAutomatico, _: &str) -> Box<dyn StatoDistributore> { non_valida!(self) }
    fn conferma_selezionato(self: Box<Self>, d: &mut DistributoreAutomatico) -> Box<dyn StatoDistributore> {
        let prodotto = d.prodotto_in_erogazione.clone().expect("Nessun prodotto in erogazione");
        let quantita = *d.prodotti.get(&prodotto).unwrap_or(&0);
        if quantita > 0 {
            println!("Prodotto {} in Erogazione", prodotto);
            Box::new(ProdottoDisponibile)
        } else {
            println!("Prodotto {} Esaurito", prodotto);
            Box::new(ProdottoEsaurito)
        }
    }
    fn cancella(self: Box<Self>, d: &mut DistributoreAutomatico) -> Box<dyn StatoDistributore> {
        println!("Operazione Cancellata");
        d.prodotto_in_erogazione = None;
        Box::new(InAttesaCarta)
    }
    fn preleva_prodotto(self: Box<Self>, _: &mut DistributoreAutomatico) -> Box<dyn StatoDistributore> { non_valida!(self) }
}

// ─── ProdottoDisponibile ─────────────────────────────────────────────────────

impl StatoDistributore for ProdottoDisponibile {
    fn inserisci_carta(self: Box<Self>, _: &mut DistributoreAutomatico) -> Box<dyn StatoDistributore> { non_valida!(self) }
    fn seleziona_prodotto(self: Box<Self>, _: &mut DistributoreAutomatico, _: &str) -> Box<dyn StatoDistributore> { non_valida!(self) }
    fn conferma_selezionato(self: Box<Self>, _: &mut DistributoreAutomatico) -> Box<dyn StatoDistributore> { non_valida!(self) }
    fn cancella(self: Box<Self>, _: &mut DistributoreAutomatico) -> Box<dyn StatoDistributore> { non_valida!(self) }
    fn preleva_prodotto(self: Box<Self>, d: &mut DistributoreAutomatico) -> Box<dyn StatoDistributore> {
        let prodotto = d.prodotto_in_erogazione.take().expect("Nessun prodotto in erogazione");
        if let Some(q) = d.prodotti.get_mut(&prodotto) {
            *q -= 1;
        }
        println!("Erogazione del {}", prodotto);
        Box::new(InAttesaCarta)
    }
}

// ─── ProdottoEsaurito ────────────────────────────────────────────────────────

impl StatoDistributore for ProdottoEsaurito {
    fn inserisci_carta(self: Box<Self>, _: &mut DistributoreAutomatico) -> Box<dyn StatoDistributore> { non_valida!(self) }
    fn seleziona_prodotto(self: Box<Self>, _: &mut DistributoreAutomatico, _: &str) -> Box<dyn StatoDistributore> { non_valida!(self) }
    fn conferma_selezionato(self: Box<Self>, _: &mut DistributoreAutomatico) -> Box<dyn StatoDistributore> { non_valida!(self) }
    fn cancella(self: Box<Self>, d: &mut DistributoreAutomatico) -> Box<dyn StatoDistributore> {
        println!("Operazione Cancellata");
        d.prodotto_in_erogazione = None;
        Box::new(InAttesaCarta)
    }
    fn preleva_prodotto(self: Box<Self>, _: &mut DistributoreAutomatico) -> Box<dyn StatoDistributore> { non_valida!(self) }
}
