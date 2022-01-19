use std::fmt;

#[derive(Debug,Clone,PartialEq)]
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

impl fmt::Display for Risposta{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f,"{}",self.testo)
        .expect("qualcosa ha sminchiato tutto");
        fmt::Result::Ok(())
    } 
}
