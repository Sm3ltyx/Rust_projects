fn main() {
    let (moltiplicatore, mut punteggio) = (1.25, 5.0);
    let mut punteggio_totale : f32 = punteggio * moltiplicatore;
    println!("Il punteggio di A1 e': {}", punteggio_totale);
    {
        let moltiplicatore_nascosto = 2.0;
        punteggio_totale = punteggio * moltiplicatore * moltiplicatore_nascosto;
        println!("Il punteggio di A2 e': {}", punteggio_totale);
    }
    //moltiplicatore_nascosto non e' raggiungibile al di fuori del blocco
    punteggio = 32.5;
    punteggio_totale = punteggio * moltiplicatore;
    println!("Il punteggio di B e': {}", punteggio_totale);
    punteggio = 69.0;
    punteggio_totale = punteggio * moltiplicatore;
    println!("Il punteggio di C e': {}", punteggio_totale);
    println!("Il punteggio della funzione1 e': {}", calcola_punteggio(15.0, 4.75));
    println!("Il punteggio della funzione2 e': {}", calcola_punteggio(60.0, 0.375));
}       

fn calcola_punteggio(punti: f32, moltiplicatore: f32) -> f32 {
    let punteggio_totale = punti * moltiplicatore;
    return punteggio_totale;
}