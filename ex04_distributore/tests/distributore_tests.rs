use ex04_distributore::DistributoreAutomatico;
use std::collections::HashMap;

fn distributore_con_prodotti() -> DistributoreAutomatico {
    let mut prodotti = HashMap::new();
    prodotti.insert("Acqua".to_string(), 3);
    prodotti.insert("Caffe".to_string(), 0);
    DistributoreAutomatico::new(prodotti)
}

// ── Happy path ────────────────────────────────────────────────────────────────

#[test]
fn test_happy_path_preleva_prodotto() {
    let mut d = distributore_con_prodotti();
    d.forza_carta_accettata();
    d.seleziona_prodotto("Acqua");
    assert_eq!(d.prodotto_in_erogazione, Some("Acqua".to_string()));
    d.conferma_selezionato();
    d.preleva_prodotto();
    // Dopo il prelievo la quantità scende di 1 e prodotto_in_erogazione è None.
    assert_eq!(d.prodotti["Acqua"], 2);
    assert_eq!(d.prodotto_in_erogazione, None);
}

// ── Prodotto esaurito ─────────────────────────────────────────────────────────

#[test]
fn test_prodotto_esaurito_poi_cancella() {
    let mut d = distributore_con_prodotti();
    d.forza_carta_accettata();
    d.seleziona_prodotto("Caffe");
    d.conferma_selezionato(); // → ProdottoEsaurito
    // Solo cancella è valida; le altre operazioni stampano "Operazione non Valida"
    d.preleva_prodotto();     // non valida
    d.cancella();             // → InAttesaCarta
    assert_eq!(d.prodotto_in_erogazione, None);
}

// ── Prodotto non presente ─────────────────────────────────────────────────────

#[test]
fn test_prodotto_non_presente_rimane_carta_accettata() {
    let mut d = distributore_con_prodotti();
    d.forza_carta_accettata();
    d.seleziona_prodotto("Birra"); // non esiste → rimane CartaAccettata
    assert_eq!(d.prodotto_in_erogazione, None);
    // Può ancora selezionare un prodotto valido
    d.seleziona_prodotto("Acqua");
    assert_eq!(d.prodotto_in_erogazione, Some("Acqua".to_string()));
}

// ── Cancella in CartaAccettata ────────────────────────────────────────────────

#[test]
fn test_cancella_da_carta_accettata() {
    let mut d = distributore_con_prodotti();
    d.forza_carta_accettata();
    d.cancella(); // → InAttesaCarta
    assert_eq!(d.prodotto_in_erogazione, None);
}

// ── Cancella in SelezionatoProdotto ──────────────────────────────────────────

#[test]
fn test_cancella_da_selezionato_prodotto() {
    let mut d = distributore_con_prodotti();
    d.forza_carta_accettata();
    d.seleziona_prodotto("Acqua");
    d.cancella(); // → InAttesaCarta
    assert_eq!(d.prodotto_in_erogazione, None);
}

// ── Operazioni non valide ─────────────────────────────────────────────────────

#[test]
fn test_operazioni_invalide_in_attesa_carta() {
    // In stato InAttesaCarta tutte le operazioni tranne inserisci_carta sono invalide.
    // Verifichiamo che il distributore resti coerente (prodotto_in_erogazione sempre None).
    let mut d = distributore_con_prodotti();
    d.seleziona_prodotto("Acqua");
    d.conferma_selezionato();
    d.cancella();
    d.preleva_prodotto();
    assert_eq!(d.prodotto_in_erogazione, None);
}

#[test]
fn test_preleva_senza_conferma_non_valido() {
    let mut d = distributore_con_prodotti();
    d.forza_carta_accettata();
    d.seleziona_prodotto("Acqua");
    // In SelezionatoProdotto preleva è non valido
    d.preleva_prodotto();
    // Il prodotto in erogazione è ancora impostato (lo stato non è cambiato)
    assert_eq!(d.prodotto_in_erogazione, Some("Acqua".to_string()));
}

// ── Quantità decrementa correttamente ────────────────────────────────────────

#[test]
fn test_quantita_decrementa_ad_ogni_prelievo() {
    let mut d = distributore_con_prodotti();
    for expected in [2i32, 1, 0] {
        d.forza_carta_accettata();
        d.seleziona_prodotto("Acqua");
        d.conferma_selezionato();
        d.preleva_prodotto();
        assert_eq!(d.prodotti["Acqua"], expected);
    }
}
