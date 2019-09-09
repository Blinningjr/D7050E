/***  
 *  Tests for parser functions.
 *
 *  Too run: 'cargo test'
 */

#[cfg(test)]
mod tests {    
    /**
     *  Import parser function parse_expr.
     */
    #[allow(unused_imports)]
    use crate::parser::parse_expr;

    /**
     *  Import parser function math_expr_eval.
     */
    #[allow(unused_imports)]
    use crate::parser::math_expr_eval;

    /**
     *  Import enum Expr.
     */
    #[allow(unused_imports)]
    use crate::parser::Expr::{
        Num,
        BinOp
    };

    /**
     *  Import enum Op.
     */
    #[allow(unused_imports)]
    use crate::parser::Op::{
        Add,        // "+"
        Sub,        // "-"
        Div,        // "/"
        Multi,      // "*"
        Mod,        // "%"
        And,        // "&&"
        Or,         // "||"
        Not,        // "!"
        Equal,      // "=="
        NotEq,      // "!="
        LessThen,   // "<"
        LargThen,   // ">"
        LessEqThen, // "<="
        LargEqThen, // ">="  
    };

    /**
     *  Test parsing singel int.
     */
    #[test]
    fn test_parse_int() {
        assert!(parse_expr("2") == Ok(("", Num(2))));
        assert!(parse_expr("1a").is_ok());
    }

    /**
     *  Test parsing addition.
     */
    #[test]
    fn test_parse_add() {
        let test_val = "4 + 2";
        let expec = Ok(("", BinOp(Box::new(Num(4)), Add, Box::new(Num(2)))));
        let expr = parse_expr(test_val);
        assert_eq!(expr, expec);
        assert_eq!(math_expr_eval(expr.unwrap().1).unwrap(), 6)
    }

    /**
     *  Test parsing subtraction.
     */
    #[test]
    fn test_parse_sub() {
        let test_val = "4 - 2";
        let expec = Ok(("", BinOp(Box::new(Num(4)), Sub, Box::new(Num(2)))));
        let expr = parse_expr(test_val);
        
        assert_eq!(expr, expec);
        assert_eq!(math_expr_eval(expr.unwrap().1).unwrap(), 2)
    }

    /**
     *  Test parsing divition.
     */
    #[test]
    fn test_parse_div() {
        let test_val = "4 / 2";
        let expec = Ok(("", BinOp(Box::new(Num(4)), Div, Box::new(Num(2)))));
        let expr = parse_expr(test_val);

        assert_eq!(expr, expec);
        assert_eq!(math_expr_eval(expr.unwrap().1).unwrap(), 2)
    }

    /**
     *  Test parsing multiplication.
     */
    #[test]
    fn test_parse_multi() {
        let test_val = "4 * 2";
        let expec = Ok(("", BinOp(Box::new(Num(4)), Multi, Box::new(Num(2)))));
        let expr = parse_expr(test_val);

        assert_eq!(expr, expec);
        assert_eq!(math_expr_eval(expr.unwrap().1).unwrap(), 8)
    }

    /**
     *  Test parsing modulus.
     */
    #[test]
    fn test_parse_mod() {
        let test_val = "4 % 2";
        let expec = Ok(("", BinOp(Box::new(Num(4)), Mod, Box::new(Num(2)))));
        let expr = parse_expr(test_val);

        assert_eq!(expr, expec);
        assert_eq!(math_expr_eval(expr.unwrap().1).unwrap(), 0)
    }

    /**
     *  Test parsing and.
     */
    #[test]
    fn test_parse_and() {
        let expec = Ok(("", BinOp(Box::new(Num(4)), And, Box::new(Num(2)))));
        assert_eq!(parse_expr("4 && 2"), expec);
    }

    /**
     *  Test parsing or.
     */
    #[test]
    fn test_parse_or() {
        let expec = Ok(("", BinOp(Box::new(Num(4)), Or, Box::new(Num(2)))));
        assert_eq!(parse_expr("4 || 2"), expec);
    }

    /**
     *  Test parsing not.
     */
    #[test]
    fn test_parse_not() {
        let expec = Ok(("", BinOp(Box::new(Num(4)), Not, Box::new(Num(2)))));
        assert_eq!(parse_expr("4 ! 2"), expec);
    }

    /**
     *  Test parsing equal.
     */
    #[test]
    fn test_parse_equal() {
        let expec = Ok(("", BinOp(Box::new(Num(4)), Equal, Box::new(Num(2)))));
        assert_eq!(parse_expr("4 == 2"), expec);
    }

    /**
     *  Test parsing not equal.
     */
    #[test]
    fn test_parse_not_eq() {
        let expec = Ok(("", BinOp(Box::new(Num(4)), NotEq, Box::new(Num(2)))));
        assert_eq!(parse_expr("4 != 2"), expec);
    }

    /**
     *  Test parsing lesser then.
     */
    #[test]
    fn test_parse_less_then() {
        let expec = Ok(("", BinOp(Box::new(Num(4)), LessThen, Box::new(Num(2)))));
        assert_eq!(parse_expr("4 < 2"), expec);
    }

    /**
     *  Test parsing larger then.
     */
    #[test]
    fn test_parse_larg_then() {
        let expec = Ok(("", BinOp(Box::new(Num(4)), LargThen, Box::new(Num(2)))));
        assert_eq!(parse_expr("4 > 2"), expec);
    }

    /**
     *  Test parsing lesser equal then.
     */
    #[test]
    fn test_parse_less_eq_then() {
        let expec = Ok(("", BinOp(Box::new(Num(4)), LessEqThen, Box::new(Num(2)))));
        assert_eq!(parse_expr("4 <= 2"), expec);
    }

    /**
     *  Test parsing larger equal then.
     */
    #[test]
    fn test_parse_larg_eq_then() {
        let expec = Ok(("", BinOp(Box::new(Num(4)), LargEqThen, Box::new(Num(2)))));
        assert_eq!(parse_expr("4 >= 2"), expec);
    }
}
