use std::fmt;

#[derive(Debug,Clone)]
pub struct Risposta{
    pub testo: String,
    pub tipo: bool
}

impl Risposta{
    pub fn new(testo: &str, tipo: bool) -> Risposta
    {
        Risposta{
            testo: String::from(testo),
            tipo: tipo
        }
    } 
}

impl PartialEq for Risposta{
    fn eq(&self, other: &Risposta) -> bool
    {
        if other.testo == self.testo &&
            other.tipo == self.tipo { true }
        else { false }
    }
}

impl fmt::Display for Risposta{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f,"{}",self.testo)
    } 
}
