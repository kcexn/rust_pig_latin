extern crate regex;

use regex::Regex;

pub struct OriginalWord { 
    pub word: String, 
} 

impl OriginalWord { 
    pub fn new(mut args: std::env::Args) -> Result<OriginalWord, &'static str> { 
        args.next();  
        let word = match args.next() { 
            Some(arg) => arg, 
            None => return Err("Didn't get a query string"), 
        }; 
        Ok( OriginalWord{word} ) 
    } 
    
    fn get(self) -> String { 
        self.word 
    } 
} 

pub fn translate_to_pig_latin(word: OriginalWord) -> String { 
    let mut original_word = word.get(); 
    let re = Regex::new("^[aeiouAEIOU]").unwrap(); 
    if re.is_match(&original_word) {
        original_word.push_str("ay");
        original_word 
    } else {
        let re = Regex::new(r"^([bcdfghjklmnpqrstvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ]+)").unwrap(); 
        // Performing a deep clone here as I need two mutable references
        // to a single variable.
        // I can do this with Refs but I don't really want to put in the effort
        // and the performance hit is not important.
        let mut new_word = original_word.clone();
        let caps = re.captures(&original_word).unwrap(); 
        new_word.push_str(caps.get(1).unwrap().as_str()); 
        new_word.push_str("ay");
        // The regex library returns a smart pointer here
        // only god knows why.
        let result = re.replace( &new_word, "" );
        // The regex smart pointer seems to be happy
        // to do some weird coercion here???
        // I don't really understand why this works.
        // String::from(T: Cow) doesn't make sense to me but ok.
        String::from(result) 
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_translate_to_pig_latin() {
        // For words that begin with consonant sounds, all letters before the initial vowel are
        // placed at the end of the word sequence. Then, "ay" is added, as in the following
        // examples:
    
        assert_eq!( String::from("ananabay"), super::translate_to_pig_latin( super::OriginalWord{ word: String::from("banana") } ));
        assert_eq!( String::from("ashtray"), super::translate_to_pig_latin( super::OriginalWord{ word: String::from("trash") } ));
        assert_eq!( String::from("appyhay"), super::translate_to_pig_latin( super::OriginalWord{ word: String::from("happy") } ));
        assert_eq!( String::from("uckday"), super::translate_to_pig_latin( super::OriginalWord{ word: String::from("duck") } ));
        assert_eq!( String::from("opestday"), super::translate_to_pig_latin( super::OriginalWord{ word: String::from("dopest") } ));
        assert_eq!( String::from("emay"), super::translate_to_pig_latin( super::OriginalWord{ word: String::from("me") } ));
        assert_eq!( String::from("ootay"), super::translate_to_pig_latin( super::OriginalWord{ word: String::from("too") } ));
        assert_eq!( String::from("illway"), super::translate_to_pig_latin( super::OriginalWord{ word: String::from("will") } ));
        assert_eq!( String::from("oistmay"), super::translate_to_pig_latin( super::OriginalWord{ word: String::from("moist") } ));
        assert_eq!( String::from("etway"), super::translate_to_pig_latin( super::OriginalWord{ word: String::from("wet") } ));
        assert_eq!( String::from("ealray"), super::translate_to_pig_latin( super::OriginalWord{ word: String::from("real") } ));
        assert_eq!( String::from("implesay"), super::translate_to_pig_latin( super::OriginalWord{ word: String::from("simple") } ));
        assert_eq!( String::from("aysay"), super::translate_to_pig_latin( super::OriginalWord{ word: String::from("say") } ));
        assert_eq!( String::from("ordsway"), super::translate_to_pig_latin( super::OriginalWord{ word: String::from("words") } ));
        // When words begin with consonant clusters (multiple consonants that form one sound), the
        // whole sound is added to the end when speaking or writing.
        assert_eq!( String::from("eerschay"), super::translate_to_pig_latin( super::OriginalWord{ word: String::from("cheers") } ));
        assert_eq!( String::from("eshshay"), super::translate_to_pig_latin( super::OriginalWord{ word: String::from("shesh") } ));
        assert_eq!( String::from("ilesmay"), super::translate_to_pig_latin( super::OriginalWord{ word: String::from("smile") } ));
        assert_eq!( String::from("ingstray"), super::translate_to_pig_latin( super::OriginalWord{ word: String::from("string") } ));
        assert_eq!( String::from("anksthay"), super::translate_to_pig_latin( super::OriginalWord{ word: String::from("thanks") } ));
        assert_eq!( String::from("upidstay"), super::translate_to_pig_latin( super::OriginalWord{ word: String::from("stupid") } ));
        assert_eq!( String::from("oveglay"), super::translate_to_pig_latin( super:: OriginalWord{ word: String::from("glove") } ));
        // For words that begin with vowel sounds, one just adds "way" or "yay" to the end (or just
        // "ay"). Examples are
        assert_eq!( String::from("eatay"), super::translate_to_pig_latin( super::OriginalWord{ word: String::from("eat") } ));
        assert_eq!( String::from("omeleteay"), super::translate_to_pig_latin( super::OriginalWord{ word: String::from("omelete") } ));
        assert_eq!( String::from("areay"), super::translate_to_pig_latin( super::OriginalWord{ word: String::from("are") } ));
        assert_eq!( String::from("eggay"), super::translate_to_pig_latin( super::OriginalWord{ word: String::from("egg") } ));
        assert_eq!( String::from("explainay"), super::translate_to_pig_latin( super::OriginalWord{ word: String::from("explain") } ));
        assert_eq!( String::from("alwaysay"), super::translate_to_pig_latin( super::OriginalWord{ word: String::from("always") } ));
        assert_eq!( String::from("endsay"), super::translate_to_pig_latin( super::OriginalWord{ word: String::from("ends") } ));
    }
}
