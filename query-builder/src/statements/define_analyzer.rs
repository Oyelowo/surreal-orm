/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use std::fmt::{self, Display};

use crate::{BindingsList, Buildable, Erroneous, ErrorList, Parametric, Queryable};

#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub enum Tokenizer {
    Blank,
    Camel,
    Class,
    Punct,
}

impl Display for Tokenizer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tokenizer::Blank => write!(f, "Blank"),
            Tokenizer::Camel => write!(f, "Camel"),
            Tokenizer::Class => write!(f, "Class"),
            Tokenizer::Punct => write!(f, "Punct"),
        }
    }
}

#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub enum SnowballLanguage {
    Arabic,
    Danish,
    Dutch,
    English,
    French,
    German,
    Greek,
    Hungarian,
    Italian,
    Norwegian,
    Portuguese,
    Romanian,
    Russian,
    Spanish,
    Swedish,
    Tamil,
    Turkish,
}

impl Display for SnowballLanguage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SnowballLanguage::Arabic => write!(f, "arabic"),
            SnowballLanguage::Danish => write!(f, "danish"),
            SnowballLanguage::Dutch => write!(f, "dutch"),
            SnowballLanguage::English => write!(f, "english"),
            SnowballLanguage::French => write!(f, "french"),
            SnowballLanguage::German => write!(f, "german"),
            SnowballLanguage::Greek => write!(f, "greek"),
            SnowballLanguage::Hungarian => write!(f, "hungarian"),
            SnowballLanguage::Italian => write!(f, "italian"),
            SnowballLanguage::Norwegian => write!(f, "norwegian"),
            SnowballLanguage::Portuguese => write!(f, "portuguese"),
            SnowballLanguage::Romanian => write!(f, "romanian"),
            SnowballLanguage::Russian => write!(f, "russian"),
            SnowballLanguage::Spanish => write!(f, "spanish"),
            SnowballLanguage::Swedish => write!(f, "swedish"),
            SnowballLanguage::Tamil => write!(f, "tamil"),
            SnowballLanguage::Turkish => write!(f, "turkish"),
        }
    }
}

#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub enum AnalyzerFilter {
    Ascii,
    Edgengram(u32, u32),
    Lowercase,
    Snowball(SnowballLanguage),
    Uppercase,
}

impl Display for AnalyzerFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnalyzerFilter::Ascii => write!(f, "ascii"),
            AnalyzerFilter::Edgengram(min, max) => write!(f, "edgengram({}, {})", min, max),
            AnalyzerFilter::Lowercase => write!(f, "lowercase"),
            AnalyzerFilter::Snowball(lang) => write!(f, "snowball({})", lang),
            AnalyzerFilter::Uppercase => write!(f, "uppercase"),
        }
    }
}

/// `DEFINE ANALYZER` statement
#[derive(Clone, Debug)]
pub struct DefineAnalyzerStatement {
    name: String,
    tokenizers: Vec<Tokenizer>,
    filters: Vec<AnalyzerFilter>,
    bindings: BindingsList,
    errors: ErrorList,
}

/// Create a new `DefineAnalyzerStatement`
/// ```
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::statements::{define_analyzer, AnalyzerFilter, SnowballLanguage,Tokenizer};
///
/// let mut analyzer = define_analyzer("ascii").tokenizers([Tokenizer::Class]).filters([
///    AnalyzerFilter::Lowercase,
///    AnalyzerFilter::Ascii,
///    AnalyzerFilter::Edgengram(2, 15),
///    AnalyzerFilter::Snowball(SnowballLanguage::English),
/// ]);
/// ```
pub fn define_analyzer(name: &str) -> DefineAnalyzerStatement {
    DefineAnalyzerStatement {
        name: name.to_string(),
        tokenizers: vec![],
        filters: vec![],
        bindings: vec![],
        errors: vec![],
    }
}

impl DefineAnalyzerStatement {
    /// Add a tokenizer to the analyzer
    pub fn tokenizers<I>(mut self, tokenizers: I) -> Self
    where
        I: IntoIterator<Item = Tokenizer>,
    {
        self.tokenizers
            .extend(tokenizers.into_iter().collect::<Vec<_>>());
        self
    }

    /// Add a filter to the analyzer
    pub fn filters<I>(mut self, filters: I) -> Self
    where
        I: IntoIterator<Item = AnalyzerFilter>,
    {
        self.filters.extend(filters.into_iter().collect::<Vec<_>>());
        self
    }
}

impl Queryable for DefineAnalyzerStatement {}
impl Erroneous for DefineAnalyzerStatement {
    fn get_errors(&self) -> ErrorList {
        self.errors.to_vec()
    }
}

impl Parametric for DefineAnalyzerStatement {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl Buildable for DefineAnalyzerStatement {
    fn build(&self) -> String {
        let mut query = format!("DEFINE ANALYZER {}", self.name);

        if !self.tokenizers.is_empty() {
            let tokenizers_str = self
                .tokenizers
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(",");
            query = format!("{query} TOKENIZERS {}", tokenizers_str);
        }

        if !self.filters.is_empty() {
            let filters_str = self
                .filters
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(",");
            query = format!("{query} FILTERS {}", filters_str);
        }

        query.push(';');
        query
    }
}

impl Display for DefineAnalyzerStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_define_analyzer_statement() {
        use AnalyzerFilter::*;
        use Tokenizer::*;

        let analyzer = define_analyzer("ascii").tokenizers([Class]).filters([
            Lowercase,
            Ascii,
            Edgengram(2, 15),
            Snowball(SnowballLanguage::English),
        ]);

        assert_eq!(
            analyzer.build(),
            "DEFINE ANALYZER ascii TOKENIZERS Class FILTERS lowercase,ascii,edgengram(2, 15),snowball(english);"
        );
    }
}
