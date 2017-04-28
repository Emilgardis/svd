use xmltree;
use std::str;
use std::num;
error_chain! {
    foreign_links {
        XmlParseError(xmltree::ParseError);
        StrParseBoolError(str::ParseBoolError);
        NumParseIntError(num::ParseIntError);

    }
    errors {
        MissingMandatory(t: String) {
            description("mandatory element missing")
            display("mandatory element missing: ´{}´", t)
        }
    }
}
