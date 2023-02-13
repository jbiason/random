//! Foam format parser.

use std::str::FromStr;

use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::IResult;

#[derive(Debug, Clone, PartialEq)]
struct Comment<'a> {
    content: &'a str,
}

// impl FromStr for Comment {
//     type Err = ();

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         Ok(Self {
//             comment: s.trim().to_string(),
//         })
//     }
// }

fn multiline_comment(input: &str) -> IResult<&str, Comment> {
    let (input, _) = tag("/*")(input)?;
    let (input, content) = take_until("*/")(input)?;
    let (input, _) = tag("*/")(input)?;

    Ok((
        input,
        Comment {
            content: content.trim(),
        },
    ))
}

fn singleline_comment(input: &str) -> IResult<&str, Comment> {
    let (input, _) = tag("//")(input)?;
    let (input, content) = is_not("\n\r")(input)?;

    Ok((
        input,
        Comment {
            content: content.trim(),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiline_comment() {
        let text = "/* this is comment */";
        let result = multiline_comment(text);
        assert_eq!(
            result,
            Ok((
                "",
                Comment {
                    content: "this is comment"
                }
            ))
        )
    }

    #[test]
    fn test_singleline_comment() {
        let text = "// this is comment";
        let result = singleline_comment(text);
        assert_eq!(
            result,
            Ok((
                "",
                Comment {
                    content: "this is comment"
                }
            ))
        )
    }
}
