use std::collections::HashMap;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyWordType {
    Let,
    Print,
    Input,
    If,
    Then,
    Goto,
    Random,
    End,
}

pub struct SyntaxDict {
    pub keywords: HashMap<String, KeyWordType>
}

#[allow(dead_code)]
impl SyntaxDict {
    fn default_basic() -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("LET".to_string(), KeyWordType::Let);
        keywords.insert("PRINT".to_string(), KeyWordType::Print);
        keywords.insert("INPUT".to_string(), KeyWordType::Input);
        keywords.insert("IF".to_string(), KeyWordType::If);
        keywords.insert("THEN".to_string(), KeyWordType::Then);
        keywords.insert("GOTO".to_string(), KeyWordType::Goto);
        keywords.insert("RANDOM".to_string(), KeyWordType::Random);
        keywords.insert("END".to_string(), KeyWordType::End);
        Self { keywords }
    }


    fn russian_style() -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("ПУСТЬ".to_string(), KeyWordType::Let);
        keywords.insert("ПЕЧАТЬ".to_string(), KeyWordType::Print);
        keywords.insert("ВВОД".to_string(), KeyWordType::Input);
        keywords.insert("ЕСЛИ".to_string(), KeyWordType::If);
        keywords.insert("ТО".to_string(), KeyWordType::Then);
        keywords.insert("ИДИ".to_string(), KeyWordType::Goto);
        keywords.insert("РАНДОМ".to_string(), KeyWordType::Random);
        keywords.insert("СТОП".to_string(), KeyWordType::End);
        Self { keywords }
    }

    fn emoji_style() -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("✍".to_string(), KeyWordType::Let);
        keywords.insert("🖨".to_string(), KeyWordType::Print);
        keywords.insert("⌨".to_string(), KeyWordType::Input);
        keywords.insert("❓".to_string(), KeyWordType::If);
        keywords.insert("➡".to_string(), KeyWordType::Then);
        keywords.insert("🚀".to_string(), KeyWordType::Goto);
        keywords.insert("🎲".to_string(), KeyWordType::Random);
        keywords.insert("⛔".to_string(), KeyWordType::End);
        Self { keywords }
    }
    fn crab_style() -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("🦀".to_string(), KeyWordType::Let);
        keywords.insert("📢".to_string(), KeyWordType::Print);
        keywords.insert("⚓".to_string(), KeyWordType::Input);
        keywords.insert("🌊".to_string(), KeyWordType::If);
        keywords.insert("🚢".to_string(), KeyWordType::Then);
        keywords.insert("🚀".to_string(), KeyWordType::Goto);
        keywords.insert("🎲".to_string(), KeyWordType::Random);
        keywords.insert("⛔".to_string(), KeyWordType::End);
        Self { keywords }
    }
    fn brainrot_style() -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("КАРЛОООООО".to_string(), KeyWordType::Let);
        keywords.insert("ПОТХОТСПОТ".to_string(), KeyWordType::Print);
        keywords.insert("КОКФАНТОЭЛЕФАНТО".to_string(), KeyWordType::Input);
        keywords.insert("ЧАИМАЭСТРО".to_string(), KeyWordType::If);
        keywords.insert("ТОНГТОНГТОНГСАХУР".to_string(), KeyWordType::Then);
        keywords.insert("ТРАЛАЛЭЛОТРАЛАЛА".to_string(), KeyWordType::Goto);
        keywords.insert("БРРБРРПАТАПИМ".to_string(), KeyWordType::Random);
        keywords.insert("БОНЕКААМБАЛАБУ".to_string(), KeyWordType::End);
        Self { keywords }
    }

    pub fn get_dict(name_of_dict: &str) -> SyntaxDict {
        match name_of_dict {
            "RUSSIAN" => Self::russian_style(),
            "EMOJI" => Self::emoji_style(),
            "CRAB" => Self::crab_style(),
            "BRAINROT" => Self::brainrot_style(),
            _ => Self::default_basic(),
        }
    }
}