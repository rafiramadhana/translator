pub trait Translator {
    fn translate(&self, text: String) -> String;
}

pub struct EnTranslator {}

impl Translator for EnTranslator {
    fn translate(&self, text: String) -> String {
        format!("EnTranslator says `{}`", text)
    }
}

pub struct JavaneseTranslator {}

impl Translator for JavaneseTranslator {
    fn translate(&self, text: String) -> String {
        format!("JavaneseTranslator says `{}`", text)
    }
}
