// provo solo a fare la logica di domande e risposte

//#[allow(unused)]
//enum Esito{
//    Esatto,
//    Sbagliato,
//}

//fn tenta(tentativo: i32, corretto: i32) -> Esito
//{
//    if tentativo == corretto {
//        Esito::Esatto
//    } else {
//        Esito::Sbagliato
//    }
//        
//}

//#[test]
//#[ignore]
//fn test1()
//{
//
//    let r1 = Risposta::new("si", false);
//    let r2 = Risposta::new("no", false);
//    let r3 = Risposta::new("com a sort", true);
//
//    let risposte = vec!(r1,r2,r3);
//    let dom = Domanda::new("mamt fasc l bucchin?",risposte);
//
//    println!("{}",dom);
//}
//#[test]
//#[ignore]
//fn rtest(){
//    let mut vet = vec!(1,2,3,4,5,6,7,8,9,10);
//    randomizzavec(&mut vet);    
//    for element in vet
//    {
//        print!("{}", element);
//    }
//}
//#[test]
//fn test2()
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
//        println!("{}",domanda);
//    }
//}

//#[test]
//#[ignore]
//fn testmanuale1()
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
//    let domvet = vec!(dom,dom2);
//    //println!("{}",domvet[1].risposte[1].tipo);
//    for domanda in &domvet
//    {
//        println!("le risposte totali sono: {}",domanda.risposte.len());
//        println!("{}",domanda);
//        loop {
//            risp = "".to_string(); // questo perche altrimenti rimane il \n nel buffer in input credo
//            println!("metti la risposta");
//            io::stdin().read_line(&mut risp).expect("errore bro");
//            let risp_num: usize = risp.trim().parse().expect("devi mettere un numero valido");
//            if domanda.risposte[risp_num].tipo { break () }
//            else { println!("noooo ma che dici fraa"); }
//        }
//    }
//}
pub mod domanda;
pub mod risposta;

fn main() {
    let mut domande = Vec::<quiz::domanda::Domanda>::new();
    quiz::setup(&mut domande);
    quiz::run(&domande);
    // leggo il contenuto del file
    // lo organizzo in un vettore di domande con relative risposte
    // mescolo il vettore delle risposte per ogni domanda ok
    // mescolo il vettore delle domande ok
    // per ogni elemento di domande: ok
    //      stampo la domanda ok
    //      prendo il tentativo ok 
    //      solo se e giusto passo alla prossima domanda ok 
    // esco
}
