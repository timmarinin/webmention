use crate::error::WebmentionError;
use http_types::headers::HeaderValues;
use nom::{
    branch::alt,
    bytes::complete::is_not,
    character::complete::char,
    combinator::map_res,
    multi::{many0, separated_list0},
    sequence::{delimited, preceded, tuple},
    IResult,
};
use std::convert::TryFrom;

pub fn find_first_rel_webmention_in_link_header(header: Option<&HeaderValues>) -> Option<String> {
    if header.is_none() {
        return None;
    }

    for header_value in header.unwrap().into_iter() {
        let link_header: Result<LinkHeader, _> = TryFrom::try_from(header_value.as_str());
        if link_header.is_err() {
            continue;
        }

        for link in link_header.unwrap().values {
            if link.rels.iter().any(|rel| rel == "webmention") {
                return Some(link.uri_reference);
            }
        }
    }

    None
}

/// Stores data of a single link inside of the link header (dropping all params save for rel)
#[derive(Debug)]
struct LinkHeaderValue {
    uri_reference: String,
    rels: Vec<String>,
}

/// List of links inside of the link header (could contain several, according to MDN)
/// Example:
///
/// > Link: <https://marinintim.com/webmention>; option="one"; option=two; rel="webmention", <another-uri>; another="opt"; rel="stylesheet"

#[derive(Debug)]
struct LinkHeader {
    values: Vec<LinkHeaderValue>,
}

fn uri_reference(input: &str) -> IResult<&str, &str> {
    let mut parser = delimited(char('<'), is_not(">"), char('>'));

    let (input, middle) = parser(input)?;

    Ok((input, middle))
}

fn option(input: &str) -> IResult<&str, (&str, &str)> {
    let mut parser = tuple((
        is_not("="),
        char('='),
        alt((delimited(char('"'), is_not("\""), char('"')), is_not(" ;,"))),
    ));

    let (input, (key, _, value)) = parser(input)?;

    Ok((input, (key, value)))
}

fn list_of_options(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    let mut parser = many0(preceded(tuple((char(';'), char(' '))), option));

    let (input, options) = parser(input)?;

    Ok((input, options))
}

fn parsed_to_link_header_value(parsed: (&str, Vec<(&str, &str)>)) -> Result<LinkHeaderValue, ()> {
    let rels = parsed
        .1
        .iter()
        .filter(|opt| opt.0 == "rel")
        .map(|opt| opt.1.split_whitespace())
        .flatten()
        .map(|s| s.to_string())
        .collect();
    Ok(LinkHeaderValue {
        uri_reference: parsed.0.to_string(),
        rels,
    })
}

fn link(input: &str) -> IResult<&str, LinkHeaderValue> {
    map_res(
        tuple((uri_reference, list_of_options)),
        parsed_to_link_header_value,
    )(input)
}

fn parsed_to_link_header(parsed: Vec<LinkHeaderValue>) -> Result<LinkHeader, ()> {
    Ok(LinkHeader { values: parsed })
}

fn link_header(input: &str) -> IResult<&str, LinkHeader> {
    map_res(
        separated_list0(tuple((char(','), char(' '))), link),
        parsed_to_link_header,
    )(input)
}

impl TryFrom<&str> for LinkHeader {
    type Error = WebmentionError;
    fn try_from(s: &str) -> Result<LinkHeader, WebmentionError> {
        let (_remaining, link) =
            link_header(s).map_err(|_err| WebmentionError::InvalidLinkHeader(s.to_string()))?;
        Ok(link)
    }
}

#[cfg(test)]
mod test {
    use super::link;
    use super::link_header;
    use super::list_of_options;
    use super::option;
    use super::uri_reference;

    #[test]
    fn test_uri_reference() {
        let input = "<https://marinintim.com>";
        let (remaining, uri) = uri_reference(input).unwrap();
        assert_eq!(uri, "https://marinintim.com");
        assert_eq!(remaining, "");

        let input = "<https://marinintim.com";
        assert!(uri_reference(input).is_err());
    }

    #[test]
    fn test_option() {
        let input = "key=value";
        let (remaining, (key, value)) = option(input).unwrap();
        assert_eq!(key, "key");
        assert_eq!(value, "value");
        assert_eq!(remaining, "");

        let input = "weird-key=Still_value not-part-of-the-value";
        let (remaining, (key, value)) = option(input).unwrap();
        assert_eq!(key, "weird-key");
        assert_eq!(value, "Still_value");
        assert_eq!(remaining, " not-part-of-the-value");

        let input = "key=\"Quoted value with Spaces\" not relevant";
        let (remaining, (key, value)) = option(input).unwrap();
        assert_eq!(key, "key");
        assert_eq!(value, "Quoted value with Spaces");
        assert_eq!(remaining, " not relevant");
    }

    #[test]
    fn test_list_of_options() {
        let input = "; option1=value1; option2=\"value2\"";
        let (_, options) = list_of_options(input).unwrap();
        assert_eq!(options, vec![("option1", "value1"), ("option2", "value2")]);
    }

    #[test]
    fn test_link() {
        let input = "<https://marinintim.com>; rel=webmention; awesome=true";
        let (_, value) = link(input).unwrap();
        assert_eq!(value.uri_reference, "https://marinintim.com");
        assert_eq!(value.rels[0], "webmention");
    }

    #[test]
    fn test_link_header() {
        let input = "<https://marinintim.com/pingback>; rel=\"pingback\"; awesome=true, <https://marinintim.com>; rel=\"webmention\"";
        let (_, value) = link_header(input).unwrap();
        println!("{:?}", value);
        assert_eq!(value.values[1].rels[0], "webmention");
    }
}
