use peg;

peg::parser!{
    pub grammar grammar() for str {
        pub rule arithmetic() -> i64 = precedence!{
            x:(@) "+" y:@ { x + y }
            x:(@) "-" y:@ { x - y }
            --
            x:(@) "*" y:@ { x * y }
            x:(@) "/" y:@ { x / y }
            --
            x:@ "^" y:(@) { x.pow(y as u32) }
            --
            n:number() { n }
            "(" e:arithmetic() ")" { e }
        }
        rule whitespace() = quiet!{[' ' | '\n' | '\t']+}
        rule number() -> i64
            = n:$(['0'..='9']+) { n.parse().unwrap() }
    }
  }