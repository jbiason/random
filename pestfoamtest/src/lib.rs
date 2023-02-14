//! Parse a Foam file into a major structure.

use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "foam.pest"]
pub struct Foam;

#[cfg(test)]
mod test {
    use super::*;
    use pest::Parser;

    #[test]
    fn multi_comment() {
        let parse = Foam::parse(Rule::multi_comment, "/* this is comment */");
        assert!(parse.is_ok());
    }

    #[test]
    fn broken_multi() {
        let parse = Foam::parse(Rule::multi_comment, "/* bad comment");
        assert!(parse.is_err());
    }

    #[test]
    fn single_comment() {
        let parse = Foam::parse(Rule::single_comment, "// this is comment");
        assert!(parse.is_ok());
    }

    #[test]
    fn chained_comments() {
        let text = "/* this is one comment */\n// And this is another";
        let parse = Foam::parse(Rule::file, text);
        assert!(parse.is_ok(), "{:?}", parse);
    }

    #[test]
    fn identifier() {
        let parse = Foam::parse(Rule::identifier, "FoamFile");
        assert!(parse.is_ok());
        let parse = Foam::parse(Rule::identifier, "foam_file");
        assert!(parse.is_ok());
    }

    #[test]
    fn broken_identifer() {
        let parse = Foam::parse(Rule::identifier, "123");
        assert!(parse.is_err());

        let parse = Foam::parse(Rule::identifier, "asd ");
        assert!(parse.is_ok(), "{:?}", parse);
        // XXX check if the identifier lost its space.
    }

    #[test]
    fn attribution() {
        let parse = Foam::parse(Rule::attribution, "version 2.0;");
        assert!(parse.is_ok(), "{:?}", parse);
    }

    #[test]
    fn broken_attribution() {
        let parse = Foam::parse(Rule::attribution, "version 2.0");
        assert!(parse.is_err(), "{:?}", parse);
    }

    #[test]
    fn dictionary() {
        let text = "FoamFile\n{\nversion 2.0;\nformat ascii;\nclass dictionary;\nlocation system;\nobject caseSetupDict;\n}";
        let parse = Foam::parse(Rule::dictionary, text);
        assert!(parse.is_ok(), "{:#?}", parse);
    }

    #[test]
    fn dict_in_dict() {
        let text = "dict1 { dict2 { class bad; } }";
        let parse = Foam::parse(Rule::dictionary, text);
        assert!(parse.is_ok(), "{:#?}", parse);
    }

    #[test]
    fn list() {
        let text = "list_name ( 1 2 3 );";
        let parse = Foam::parse(Rule::list, text);
        assert!(parse.is_ok(), "{:#?}", parse);
    }

    #[test]
    fn sized_list() {
        let text = "list_name 3 ( 1 2 3 );";
        let parse = Foam::parse(Rule::list, text);
        assert!(parse.is_ok(), "{:#?}", parse);
    }
}
