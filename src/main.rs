mod translator;

fn main() {
    // let tr = translator::EnTranslator {};
    let tr = translator::JavaneseTranslator {};

    println!("{}", do_translate(tr, "Hello world!".to_string()))
}

fn do_translate(tr: impl translator::Translator, text: String) -> String {
    tr.translate(text)
}
