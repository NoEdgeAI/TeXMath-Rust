// tex node

/*

Exp = [Exp, ...]
    | ENumber T.Text
    | EGrouped [Exp]
    | EDelimited T.Text T.Text [InEDelimited]
    | EIdentifier T.Text
    | EMathOperator T.Text
    | ESymbol TeXSymbolType T.Text
    | ESpace Rational
    | ESub Exp Exp
    | ESuper Exp Exp
    | EOver Bool Exp Exp
    | EUnder Bool Exp Exp
    | EUnderOver Bool Exp Exp Exp
    | EPhantom Exp
    | EBoxed Exp
    | EFraction FractionType Exp Exp
    | ERoot Exp Exp
    | ESqrt Exp
    | EScaled Rational Exp
    | EArray Alignment ArrayLine
    | EText TextType T.Text
    | EStyled TextType Exp

Rational = (Int % Int)
TextType = TextNormal
        | TextBold
        | TextItalic
        | TextMonospace
        | TextSansSerif
        | TextDoubleStruck
        | TextScript
        | TextFraktur
        | TextBoldItalic
        | TextSansSerifBold
        | TextSansSerifBoldItalic
        | TextBoldScript
        | TextBoldFraktur
        | TextSansSerifItalic

FractionType = NormalFrac
            | DisplayFrac
            | InlineFrac
            | NoLineFrac

TeXSymbolType = Ord | Op | Bin | Rel | Open | Close | Pun | Accent
                    | Fence | TOver | TUnder | Alpha | BotAccent | Rad

                    
Alignment = AlignLeft | AlignRight | AlignCenter
 */


// 定义所有的节点

#[derive(PartialEq, Debug)]
pub enum TeXSymbolType {
    Ord,
    Op,
    Bin,
    Rel,
    Open,
    Close,
    Pun,
    Accent,
    Fence,
    TOver,
    TUnder,
    Alpha,
    BotAccent,
    Rad,
}

#[derive(PartialEq, Debug)]
pub enum TextType {
    TextNormal,
    TextBold,
    TextItalic,
    TextMonospace,
    TextSansSerif,
    TextDoubleStruck,
    TextScript,
    TextFraktur,
    TextBoldItalic,
    TextSansSerifBold,
    TextSansSerifBoldItalic,
    TextBoldScript,
    TextBoldFraktur,
    TextSansSerifItalic,
}

#[derive(PartialEq, Debug)]
pub enum FractionType {
    NormalFrac,
    DisplayFrac,
    InlineFrac,
    NoLineFrac,
}

#[derive(PartialEq, Debug)]
pub enum Alignment {
    AlignLeft,
    AlignRight,
    AlignCenter,
}

#[derive(PartialEq, Debug)]
pub struct Rational {
    // Rational numerator denominator
    pub numerator: i32,
    pub denominator: i32,
}

#[derive(PartialEq, Debug)]
pub enum InEDelimited {
    Left(String),
    Right(Exp),
}

pub type ArrayLines = Vec<Vec<Exp>>;

#[derive(PartialEq, Debug)]
pub enum Exp{
    ENumber(String),
    EGrouped(Vec<Exp>), // -> EGrouped[ ]
    EDelimited(String, String, Vec<InEDelimited>), // -> EDelimited[ ]
    EArray(Vec<Alignment>, Vec<ArrayLines>), // -> EArray[ ]
    EIdentifier(String),
    EMathOperator(String),
    ESymbol(TeXSymbolType, String),
    ESpace(Rational),
    // super and sub
    ESub(Box<Exp>, Box<Exp>),
    ESubsup(Box<Exp>, Box<Exp>, Box<Exp>),
    ESuper(Box<Exp>, Box<Exp>),
    
    // over and under
    EOver(bool, Box<Exp>, Box<Exp>),
    EUnder(bool, Box<Exp>, Box<Exp>),
    EUnderOver(bool, Box<Exp>, Box<Exp>, Box<Exp>),

    EPhantom(Box<Exp>),
    EBoxed(Box<Exp>),

    EFraction(FractionType, Box<Exp>, Box<Exp>),
    ERoot(Box<Exp>, Box<Exp>),
    ESqrt(Box<Exp>),
    
    EScaled(Rational, Box<Exp>),
    EText(TextType, String),
    EStyled(TextType, Vec<Exp>),
}