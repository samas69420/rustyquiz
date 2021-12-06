use std::fmt;
use std::cmp::PartialEq;
use crate::risposta::Risposta;

#[derive(Debug,Clone)]
pub struct Domanda{
    pub testo: String,
    pub risposte: Vec<Risposta>
}

impl Domanda {
    pub fn new(testo: &str, risposte: Vec<Risposta>) -> Domanda
    {
        Domanda{
            testo: String::from(testo),
            risposte: risposte 
        } 
    }
}

impl PartialEq for Domanda{
    fn eq(&self, other: &Domanda) -> bool
    {
        if other.testo == self.testo &&
            other.risposte == self.risposte { true }
        else { false }
    }
}

impl fmt::Display for Domanda{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f,"{}\n",self.testo);
        for (indice,risposta) in self.risposte.iter().enumerate()
        {
            write!(f,"{}-{}\n",indice,risposta);
        }
        fmt::Result::Ok(())
    } 
}
