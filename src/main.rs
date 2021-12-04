use quiz::{domanda,setup,run};

fn main() {
    let mut domande = Vec::<domanda::Domanda>::new();
    setup(&mut domande);
    run(&domande);
}
