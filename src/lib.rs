use std::{io, fs};
use rand;
use rand::Rng;
pub mod domanda;
pub mod risposta;
//use risposta::Risposta as Risposta;
//use domanda::Domanda as Domanda;

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

pub fn setup(domvet: &mut Vec<domanda::Domanda>) 
{
    // apro il file domande.txt con le domande e gestisco gli errori
    let content = fs::read_to_string("domande.txt").expect("non riesco a leggere il file");
    let rows: Vec<&str> = content.split('\n').collect();
    for (i,&row) in rows.iter().enumerate(){
        if row.chars().nth(0)!= None && row.chars().nth(0).unwrap()=='#'{
            let testodom = String::from(&row[1..]);
            let mut j = i;
            let mut risposte = Vec::<risposta::Risposta>::new();
            j+=1;
            while rows[j].chars().nth(0)!= None && (rows[j].chars().nth(0).unwrap()=='+' || rows[j].chars().nth(0).unwrap()=='-'){
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
            //println!("aggiungo \n {}", &domanda);
            domvet.push(domanda);
        }
        //println!("riga:{}",row);
    }
    // vado in loop
    //  leggo una riga
    //  se la riga comincia per '#' salvo tutta la riga in testo
    //  creo il vettore delle risposte
    //  fino a quanto non trovo una riga vuota
    //       leggo una riga e aggiungo la risposta
    //       se la riga e vuota vado avanti
    // randomizzo
    for domanda in &mut *domvet
    {
        randomizzavec(&mut domanda.risposte);
    }
    randomizzavec(domvet);
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

//pub fn run0()
//{
//    let r1 = Risposta::new("si", false);
//    let r2 = Risposta::new("no", false);
//    let r3 = Risposta::new("com a sort", true);
//
//    let risposte = vec!(r1,r2,r3);
//    let dom = Domanda::new("mamt fasc l bucchin?",risposte);
//    
//    let r1 = Risposta::new("pipi", false);
//    let r2 = Risposta::new("pupu", true);
//    let r3 = Risposta::new("popo", false);
//    let r4 = Risposta::new("plolol", false);
//    let r5 = Risposta::new("diokanye", false);
//
//    let risposte = vec!(r1,r2,r3,r4,r5);
//    let dom2 = Domanda::new("che hai fatto oggi?",risposte);
//    
//    let mut risp = String::new();
//
//    let mut domvet = vec!(dom,dom2);
//    for domanda in &mut domvet
//    {
//        randomizzavec(&mut domanda.risposte);
//    }
//    randomizzavec(&mut domvet);
//    for domanda in &domvet
//    {
//        let l = domanda.risposte.len();
//        println!("{}",domanda);
//        loop {
//            risp = "".to_string();
//            println!("metti la risposta");
//            io::stdin().read_line(&mut risp).expect("errore bro");
//            let risp_num: usize = risp.trim().parse().expect("devi mettere un numero valido");
//            if risp_num < l && risp_num >= 0 {
//                    if domanda.risposte[risp_num].tipo { break () }
//                    else { println!("noooo ma che dici fraa"); }
//            } else {
//                    println!("non va bene quello che hai fatto");
//                    continue;
//            }
//        }
//    }
//}
