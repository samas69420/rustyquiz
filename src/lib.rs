pub mod domanda;
pub mod risposta;

use std::{io, fs};
use rand;
use rand::Rng;

pub fn randomizzavec<T>(vet: &mut Vec<T>)
where T: Clone
{
    let l = vet.len();
    for ind in 0..l
    {
        let random_ind = rand::thread_rng().gen_range(0..l);
        if random_ind != ind {
            let temp = vet[ind].clone();
            vet[ind] = vet[random_ind].clone();
            vet[random_ind] = temp;
        }
    }
}

pub fn run(domvet: &Vec<domanda::Domanda>)
{
    let mut risp = String::new();
    for domanda in domvet
    {
        let l = domanda.risposte.len();
        println!("{}",domanda);
        loop {
            risp = "".to_string();
            println!("metti la risposta");
            io::stdin().read_line(&mut risp).expect("errore bro");
            let risp_num: usize = risp.trim().parse().expect("devi mettere un numero valido");
            if risp_num < l && risp_num >= 0 {
                    if domanda.risposte[risp_num].tipo { break () }
                    else { println!("noooo ma che dici fraa"); }
            } else {
                    println!("non va bene quello che hai fatto");
                    continue;
            }
        }
    }
}

fn domande_da_stringa(content: &str) -> Vec<domanda::Domanda>
{
    let mut domvet = Vec::new();
    let rows: Vec<&str> = content.split('\n').collect();
    for (i,&row) in rows.iter().enumerate(){
        if row.chars().nth(0)!= None && row.chars().nth(0).unwrap()=='#'{
            let testodom = String::from(&row[1..]);
            let mut j = i;
            let mut risposte = Vec::new();
            j+=1;
            while j<rows.len() && 
                rows[j].chars().nth(0)!= None &&
                (rows[j].chars().nth(0).unwrap()=='+' || rows[j].chars().nth(0).unwrap()=='-'){
               let mut tipo = false;
               if rows[j].chars().nth(0).unwrap()=='+' {
                    tipo = true;
                } else {
                    tipo = false;
                }
                let testoris = String::from(&rows[j][1..]);
                let risposta = risposta::Risposta::new(&testoris,tipo);
                j+=1;
                risposte.push(risposta);
            } 
            let domanda = domanda::Domanda::new(&testodom, risposte);
            domvet.push(domanda);
        }
    }
    domvet 
}

pub fn setup() -> Vec<domanda::Domanda>
{
    let content = fs::read_to_string("domande.txt").expect("non riesco a leggere il file");
    domande_da_stringa(&content)
}

#[cfg(test)]
mod testvari
{
    use super::*;

    #[test]
    fn restituisce_vet_di_dom() 
    {
        let content = "#domanda1\n+ris0\n-ris1\n";
        let domvet = domande_da_stringa(&content);
        
        assert_eq!(domvet.len(),1);
    }

    #[test]
    fn legge_la_domanda()
    {
        let dom = domanda::Domanda
                    {testo: "domanda1".to_string(),
                     risposte: vec!(risposta::Risposta
                                        {testo: "ris0".to_string(),
                                         tipo: true},
                                    risposta::Risposta
                                        {testo: "ris1".to_string(),
                                         tipo: false}
                                   )};
        let content = "#domanda1\n+ris0\n-ris1\n";
        let domvet = domande_da_stringa(&content);
        
        assert_eq!(domvet[0],dom);
    }

    #[test]
    fn legge_due_domande()
    {
        let content = "
#domanda1
+ris0
-ris1

#domanda2
+ris0
-ris1
";
        let dom = domanda::Domanda
                    {testo: "domanda1".to_string(),
                     risposte: vec!(risposta::Risposta
                                        {testo: "ris0".to_string(),
                                         tipo: true},
                                    risposta::Risposta
                                        {testo: "ris1".to_string(),
                                         tipo: false}
                                   )};
        let dom2 = domanda::Domanda
                    {testo: "domanda2".to_string(),
                     risposte: vec!(risposta::Risposta
                                        {testo: "ris0".to_string(),
                                         tipo: true},
                                    risposta::Risposta
                                        {testo: "ris1".to_string(),
                                         tipo: false}
                                   )};
        let domvet = domande_da_stringa(&content);
        
        assert_eq!(domvet[0],dom);
        assert_eq!(domvet[1],dom2);
    }
}


