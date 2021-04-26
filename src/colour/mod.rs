extern crate nom;

use nom::{
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    IResult,
    sequence::tuple,
};

#[derive(Debug, PartialEq)]
pub struct Colour {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    return c.is_digit(16);
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(
        take_while_m_n(2, 2, is_hex_digit),
        from_hex,
    )(input)
}

fn hex_colour(input: &str) -> IResult<&str, Colour> {
    let (input, _) = tag("#")(input)?;
    let (input, (red, green, blue)) = tuple((hex_primary, hex_primary, hex_primary))(input)?;

    Ok((input, Colour {red, green, blue}))
}

#[test]
fn parse_colour() {
    assert_eq!(
        hex_colour("#2F14DF"),
        Ok((
            "",
            Colour {
                red: 47,
                green: 20,
                blue: 223,
            }
        ))
    )
}