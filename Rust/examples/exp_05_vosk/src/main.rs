use vosk::{Model, Recognizer};

fn main() {
    let samples = vec![100, -2, 700, 30, 4, 5];
    let model_path = "/home/lucian/Documents/Models/vosk-model-en-us-0.22".to_string();
    let model = Model::new(&model_path).expect("Failed to load model");
    let mut recognizer = Recognizer::new(&model, 44100.0).unwrap();

    recognizer.set_max_alternatives(10);
    recognizer.set_words(true);
    recognizer.set_partial_words(true);

    for sample in samples.chunks(100) {
        recognizer.accept_waveform(sample);
    }

    println!("{:#?}", recognizer.final_result().multiple().unwrap());
}
