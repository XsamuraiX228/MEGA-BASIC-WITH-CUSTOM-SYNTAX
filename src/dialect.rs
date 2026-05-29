use std::collections::HashMap;
use crate::frontend::token::KeyWordType;


pub enum Dict {
    Russian,
    English,
    Crab,
    Emoji,
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
        keywords.insert("ELSE".to_string(), KeyWordType::Else);
        keywords.insert("GOTO".to_string(), KeyWordType::Goto);
        keywords.insert("RANDOM".to_string(), KeyWordType::Random);
        keywords.insert("END".to_string(), KeyWordType::End);
        keywords.insert("WHILE".to_string(), KeyWordType::While);
        keywords.insert("WEND".to_string(), KeyWordType::Wend);
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
        keywords.insert("ПОКА".to_string(), KeyWordType::While);
        keywords.insert("КОНЕЦ_ПОКА".to_string(), KeyWordType::Wend);
        keywords.insert("СТОП".to_string(), KeyWordType::End);
        Self { keywords }
    }

    fn emoji_style() -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("✍️".to_string(), KeyWordType::Let);
        keywords.insert("🖨️".to_string(), KeyWordType::Print);
        keywords.insert("⌨️".to_string(), KeyWordType::Input);
        keywords.insert("❓".to_string(), KeyWordType::If);
        keywords.insert("➡️".to_string(), KeyWordType::Then);
        keywords.insert("🚀".to_string(), KeyWordType::Goto);
        keywords.insert("🎲".to_string(), KeyWordType::Random);
        keywords.insert("🔄".to_string(), KeyWordType::While);    
        keywords.insert("⏹️".to_string(), KeyWordType::Wend);    
        keywords.insert("🏁".to_string(), KeyWordType::End);     
        Self { keywords }
    }
    fn crab_style() -> Self {
        let mut keywords = HashMap::new();
        // 🦀 CRAB RAVE EDITION 🦀
        keywords.insert("🦀".to_string(), KeyWordType::Let);      
        keywords.insert("📢".to_string(), KeyWordType::Print);    
        keywords.insert("⚓".to_string(), KeyWordType::Input);    
        keywords.insert("🌊".to_string(), KeyWordType::If);       
        keywords.insert("🚢".to_string(), KeyWordType::Then);     
        keywords.insert("🚀".to_string(), KeyWordType::Goto);     
        keywords.insert("🎲".to_string(), KeyWordType::Random);   
        keywords.insert("♻️".to_string(), KeyWordType::While);    
        keywords.insert("🛑".to_string(), KeyWordType::Wend);     
        keywords.insert("🏁".to_string(), KeyWordType::End);      
        Self { keywords }
    }
    pub fn get_dict(name_of_dict: &str) -> SyntaxDict {
        match name_of_dict {
            "RUSSIAN" => Self::russian_style(),
            "EMOJI" => Self::emoji_style(),
            "CRAB" => Self::crab_style(),
            _ => Self::default_basic(),
        }
    }
}