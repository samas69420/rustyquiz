
fn main() {
    //let mut domande = Vec::<quiz::domanda::Domanda>::new();
    //quiz::setup(&mut domande);
    let domande = quiz::setup();
    quiz::run(&domande);
}
