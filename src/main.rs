pub mod domanda;
pub mod risposta;

fn main() {
    let mut domande = Vec::<quiz::domanda::Domanda>::new();
    quiz::setup(&mut domande);
    quiz::run(&domande);
}
