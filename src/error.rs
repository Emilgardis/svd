//! SVD Errors.
//! This module defines error types and messages for SVD parsing and encoding

pub use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use xmltree::Element;

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum SVDError {
    #[error("Unknown endianness `{0}`")]
    UnknownEndian(String),
    // TODO: Needs context
    // TODO: Better name
    #[error("Expected a <{1}> tag, found none")]
    MissingTag(Element, String),
    #[error("Expected content in <{1}> tag, found none")]
    EmptyTag(Element, String),
    #[error("ParseError")]
    ParseError(Element),
    #[error("NameMismatch")]
    NameMismatch(Element),
    #[error("unknown access variant '{1}' found")]
    UnknownAccessType(Element, String),
    #[error("Bit range invalid, {1:?}")]
    InvalidBitRange(Element, InvalidBitRange),
    #[error("Unknown write constraint")]
    UnknownWriteConstraint(Element),
    #[error("Multiple wc found")]
    MoreThanOneWriteConstraint(Element),
    #[error("Unknown usage variant")]
    UnknownUsageVariant(Element),
    #[error("Expected a <{1}>, found ...")]
    NotExpectedTag(Element, String),
    #[error("Invalid RegisterCluster (expected register or cluster), found {1}")]
    InvalidRegisterCluster(Element, String),
    #[error("Invalid modifiedWriteValues variant, found {1}")]
    InvalidModifiedWriteValues(Element, String),
    #[error("The content of the element could not be parsed to a boolean value {1}: {2}")]
    InvalidBooleanValue(Element, String, core::str::ParseBoolError),
    #[error("encoding method not implemented for svd object {0}")]
    EncodeNotImplemented(String),
    #[error("Error parsing SVD XML")]
    FileParseError,
    #[error("Device must contain at least one peripheral")]
    EmptyDevice,
    #[error("Peripheral have `registers` tag, but it is empty")]
    EmptyRegisters,
    #[error("Cluster must contain at least one Register or Cluster")]
    EmptyCluster,
    #[error("Register have `fields` tag, but it is empty")]
    EmptyFields,
}

// TODO: Consider making into an Error
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InvalidBitRange {
    Syntax,
    ParseError,
    MsbLsb,
    Empty,
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum BuildError {
    #[error("`{0}` must be initialized")]
    Uninitialized(String),
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum NameError {
    #[error("Name `{0}` in tag `{1}` contains unexpected symbol")]
    Invalid(String, String),
}

pub(crate) fn check_name(name: &str, tag: &str) -> Result<()> {
    static PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new("^[_A-Za-z0-9]*$").unwrap());
    if PATTERN.is_match(name) {
        Ok(())
    } else {
        Err(NameError::Invalid(name.to_string(), tag.to_string()).into())
    }
}

pub(crate) fn check_dimable_name(name: &str, tag: &str) -> Result<()> {
    static PATTERN: Lazy<Regex> = Lazy::new(|| {
        Regex::new("^((%s)|(%s)[_A-Za-z]{1}[_A-Za-z0-9]*)|([_A-Za-z]{1}[_A-Za-z0-9]*(\\[%s\\])?)|([_A-Za-z]{1}[_A-Za-z0-9]*(%s)?[_A-Za-z0-9]*)$").unwrap()
    });
    if PATTERN.is_match(name) {
        Ok(())
    } else {
        Err(NameError::Invalid(name.to_string(), tag.to_string()).into())
    }
}
