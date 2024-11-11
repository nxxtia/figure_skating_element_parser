// #![doc = include_str!("../docs.md")]

use ::pest_derive::Parser;
use pest::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct ElementParser;

#[derive(Debug)]
pub struct ParsedElement {
    pub element_type: String,
    pub full_name: String,
    pub base_value: f32,
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Failed to parse element: {0}")]
    ParseError(String),
}

impl ParsedElement {
    pub fn from_text(text: &str) -> Result<Vec<Self>, ParseError> {
        let mut elements = Vec::new();

        for line in text.split_whitespace() {
            let pairs = ElementParser::parse(Rule::element, line)
                .map_err(|e| ParseError::ParseError(e.to_string()))?;

            for pair in pairs {
                if pair.as_rule() == Rule::element {
                    let mut element_type = String::new();
                    let mut full_name = String::new();
                    let mut base_value = 0.0;

                    for inner_pair in pair.into_inner() {
                        match inner_pair.as_rule() {
                            Rule::jump => {
                                element_type = "Jump".to_string();
                                full_name = get_full_name(inner_pair.as_str());
                                base_value = get_base_value(inner_pair.as_str());
                            }
                            Rule::spin => {
                                element_type = "Spin".to_string();
                                full_name = get_full_name(inner_pair.as_str());
                                base_value = get_base_value(inner_pair.as_str());
                            }
                            Rule::step_sequence => {
                                element_type = "Step Sequence".to_string();
                                full_name = get_full_name(inner_pair.as_str());
                                base_value = get_base_value(inner_pair.as_str());
                            }

                            Rule::death_spiral => {
                                element_type = "Death Spiral".to_string();
                                full_name = get_full_name(inner_pair.as_str());
                                base_value = get_base_value(inner_pair.as_str());
                            }
                            Rule::pair_spin => {
                                element_type = "Pair Spin".to_string();
                                full_name = get_full_name(inner_pair.as_str());
                                base_value = get_base_value(inner_pair.as_str());
                            }
                            Rule::twizzle => {
                                element_type = "Twizzle".to_string();
                                full_name = get_full_name(inner_pair.as_str());
                                base_value = get_base_value(inner_pair.as_str());
                            }
                            Rule::choreographic_element => {
                                element_type = "Choreographic Element".to_string();
                                full_name = get_full_name(inner_pair.as_str());
                                base_value = get_base_value(inner_pair.as_str());
                            }
                            _ => {}
                        }
                    }

                    elements.push(ParsedElement {
                        element_type,
                        full_name,
                        base_value,
                    });
                }
            }
        }

        Ok(elements)
    }
}


fn get_full_name(code: &str) -> String {
    match code {
        // Jump types
        "1T" => "Single Toeloop".to_string(),
        "2T" => "Double Toeloop".to_string(),
        "3T" => "Triple Toeloop".to_string(),
        "4T" => "Quad Toeloop".to_string(),
        "1S" => "Single Salchow".to_string(),
        "2S" => "Double Salchow".to_string(),
        "3S" => "Triple Salchow".to_string(),
        "4S" => "Quad Salchow".to_string(),
        "1Lo" => "Single Loop".to_string(),
        "2Lo" => "Double Loop".to_string(),
        "3Lo" => "Triple Loop".to_string(),
        "4Lo" => "Quad Loop".to_string(),
        "1F" => "Single Flip".to_string(),
        "2F" => "Double Flip".to_string(),
        "3F" => "Triple Flip".to_string(),
        "4F" => "Quad Flip".to_string(),
        "1Lz" => "Single Lutz".to_string(),
        "2Lz" => "Double Lutz".to_string(),
        "3Lz" => "Triple Lutz".to_string(),
        "4Lz" => "Quad Lutz".to_string(),
        "1A" => "Single Axel".to_string(),
        "2A" => "Double Axel".to_string(),
        "3A" => "Triple Axel".to_string(),
        "4A" => "Quad Axel".to_string(),
        
        // Spin types
        "USp" => "Upright Spin".to_string(),
        "LSp" => "Layback Spin".to_string(),
        "CSp" => "Camel Spin".to_string(),
        "SSp" => "Sit Spin".to_string(),
        "FUSp" => "Flying Upright Spin".to_string(),
        "FLSp" => "Flying Layback Spin".to_string(),
        "FCSp" => "Flying Camel Spin".to_string(),
        "FSSp" => "Flying Sit Spin".to_string(),
        "FCCSp" => "Flying Change Foot Camel Spin".to_string(),
        
        // Step sequences
        "StSq" => "Step Sequence".to_string(),
        "ChSq" => "Choreographic Sequence".to_string(),
        
        // Death spirals
        "FiDs" => "Forward Inside Death Spiral".to_string(),
        "BiDs" => "Backward Inside Death Spiral".to_string(),
        "FoDs" => "Forward Outside Death Spiral".to_string(),
        "BoDs" => "Backward Outside Death Spiral".to_string(),
        
        // Pair spins
        "PSp" => "Pair Spin".to_string(),
        "PCoSp" => "Pair Combination Spin".to_string(),
        
        // Twizzles
        "STw" => "Twizzle".to_string(),
        
        // Choreographic elements
        "ChLi1" => "Choreographic Lift".to_string(),
        "ChSp1" => "Choreographic Spinning Movement".to_string(),
        
        _ => "Unknown Element".to_string(),
    }
}

/// Returns the base cost of an element by its code
fn get_base_value(code: &str) -> f32 {
    match code {
        // Jump base values
        "1T" => 0.4,
        "2T" => 1.3,
        "3T" => 4.2,
        "4T" => 9.5,
        "1S" => 0.4,
        "2S" => 1.3,
        "3S" => 4.3,
        "4S" => 9.7,
        "1Lo" => 0.5,
        "2Lo" => 1.7,
        "3Lo" => 4.9,
        "4Lo" => 10.5,
        "1F" => 0.5,
        "2F" => 1.8,
        "3F" => 5.3,
        "4F" => 11.0,
        "1Lz" => 0.6,
        "2Lz" => 2.1,
        "3Lz" => 6.0,
        "4Lz" => 11.5,
        "1A" => 1.1,
        "2A" => 3.3,
        "3A" => 8.0,
        "4A" => 12.5,

        // Spin base values
        "USp" => 1.2,
        "LSp" => 1.5,
        "CSp" => 1.7,
        "SSp" => 1.8,
        "FUSp" => 2.0,
        "FLSp" => 2.3,
        "FCSp" => 2.5,
        "FSSp" => 2.6,
        "FCCSp" => 2.8,

        // Step sequences
        "StSq" => 1.5,
        "ChSq" => 3.0,

        // Death spirals
        "FiDs" => 1.5,
        "BiDs" => 1.6,
        "FoDs" => 1.7,
        "BoDs" => 1.8,

        // Pair spins
        "PSp" => 1.7,
        "PCoSp" => 2.5,

        // Twizzles
        "STw" => 1.0,

        // Choreographic elements
        "ChLi1" => 1.1,
        "ChSp1" => 1.1,

        _ => 0.0,
    }
}

pub fn parse_elements(text: &str) -> Result<Vec<ParsedElement>, ParseError> {
    ParsedElement::from_text(text)
}