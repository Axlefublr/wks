pub mod prelude;

pub trait CharExt {
    fn is_indentation(&self) -> bool;
}

impl CharExt for char {
    fn is_indentation(&self) -> bool {
        *self == ' ' || *self == '\t'
    }
}

pub fn alias_boundary_left(delimiter: &str) -> &str {
    match delimiter {
        "b" => "(",
        "B" => "{",
        "t" => "<",
        "s" => "[",
        "p" => "|",
        "b→" => ")",
        "B→" => "}",
        "t→" => ">",
        "s→" => "]",
        "backtick" => "`",
        "doublequote" => "\"",
        "singlequote" => "'",
        "triple-backtick" => "```",
        "triple-doublequote" => r#"""""#,
        "triple-singlequote" => "'''",
        other => other,
    }
}

pub fn boundary_left_to_right(delimiter: &str) -> &str {
    match delimiter {
        "(" => ")",
        "{" => "}",
        "<" => ">",
        "[" => "]",
        ")" => "(",
        "}" => "{",
        ">" => "<",
        "]" => "[",
        "begin" => "end",
        other => other,
    }
}
