use anyhow::{anyhow, Result};
use combine::EasyParser;
use std::fmt::Debug;

pub fn combine_parse<P, Input>(
    mut p: P,
    i: Input,
) -> Result<<P as combine::Parser<combine::easy::Stream<Input>>>::Output>
where
    Input: combine::Stream,
    P: combine::Parser<combine::easy::Stream<Input>>,
    Input::Token: PartialEq + Debug,
    Input::Range: PartialEq + Debug,
    Input::Position: Debug + Default,
{
    Ok(p.easy_parse(i)
        .map_err(|e| anyhow!("Parser error {:?}", e))?
        .0)
}
