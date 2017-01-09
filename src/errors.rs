use xmltree;
use std::str;
use std::num;
error_chain! {
    foreign_links {
        XmlError(xmltree::ParseError);
        StrError(str::ParseBoolError);
        NumError(num::ParseIntError);
    }
}
