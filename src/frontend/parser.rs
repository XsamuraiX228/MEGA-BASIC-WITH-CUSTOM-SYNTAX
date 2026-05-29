use crate::frontend::ast::Statement;
use crate::frontend::token::KeyWordType;
use super::token::{Token, CmpOp, OpType, Literal};
use super::ast::Expression;
use std::cell::Cell;
pub struct Parser<'a> {
    tokens: Vec<Token<'a>>
}

impl<'a> Parser<'a> {

    pub fn new(mut tokens: Vec<Token<'a>>, ) -> Self {
        tokens.reverse();
        Self {tokens}
    }

    fn peek(&self) -> Option<&Token<'a>> {
        self.tokens.last()
    }

    fn next(&mut self) -> Option<Token<'a>> {
        self.tokens.pop()
    }

    fn get_name(&mut self) -> Result<&'a str, String> {
        let name = match self.next() {
            Some(Token::Literal(Literal::Ident(name))) => name,
            other => return Err(format!("Expected name of variable {:?}", other))
        };
        Ok(name)
    }

    fn get_num(&mut self) -> Result<i64, String> {
        let num = match self.next() {
            Some(Token::Literal(Literal::Number(num))) => num,
            other => return Err(format!("Expected name of variable {:?}", other))
        };
        Ok(num)
    }

    fn parse_block(&mut self, stop_tokens: &[Token<'a>]) -> Result<Vec<Statement<'a>>, String> {
        let mut block_of_commads = Vec::new();
        while let Some(token) = self.peek() {
            if stop_tokens.iter().any(|t| t == token) {
                break;
            }
            if token == &Token::EOF {
                break;
            }
            match token {
                Token::Newline => {
                    self.next();
                    continue;
                }

                _ => {
                    let cmd = self.parse_command()?;
                    block_of_commads.push(cmd);
                }
            }
        }
        Ok(block_of_commads)
    }

    pub fn parse(&mut self) -> Result<Vec<Statement<'a>>, String> {
        let mut commands = Vec::new();
        let mut loop_stack: Vec<usize> = Vec::new();
        while let Some(token) = self.peek() {
            let current_idx = commands.len();
            match token {
                Token::KeyWord(KeyWordType::While) => {
                    self.next();
                    let while_idx = self.parse_while()?;

                    loop_stack.push(current_idx);
                    commands.push(while_idx);
                },
                Token::KeyWord(KeyWordType::Wend) => {
                    self.next();
                    if let Some(while_idx) = loop_stack.pop() {
                        commands.push(Statement::WEnd { start_idx: while_idx });

                        let end_destination = current_idx + 1;
                        if let Some(Statement::While { 
                            end_idx, .. }) = commands.get(while_idx) {
                                end_idx.set(end_destination);
                        }
                    } else {
                        return Err("Syntax Error: 'WEND' without matching 'WHILE'".to_string());
                    }
                    if let Some(Token::Newline) = self.peek() {
                        self.next();
                    }
                },
                Token::Newline => {
                    self.next();
                    continue;
                }
                _ => {
                    let cmd = self.parse_command()?;
                    commands.push(cmd);
                }
                
            }
        }
        if !loop_stack.is_empty() {
            println!("{:?}", loop_stack);
            return Err("Syntax Error: Expected 'WEND' for matching 'WHILE'".to_string());
        }
        Ok(commands)
    }

    fn parse_command(&mut self) -> Result<Statement<'a>, String> {
        let current_token = self.next().expect("Expected commad");
        match current_token {
            Token::KeyWord(KeyWordType::Let) => self.parse_let(),
            Token::KeyWord(KeyWordType::Print) => self.parse_print(),
            Token::KeyWord(KeyWordType::If) => self.parse_if(),
            Token::KeyWord(KeyWordType::Random) => self.parse_random(),

            Token::Mark(name) => Ok(Statement::Label { name }),
            Token::KeyWord(KeyWordType::End) => Ok(Statement::End),
            Token::KeyWord(KeyWordType::Input) => {
                let name = self.get_name()?;
                Ok(Statement::Input { name })
            }
            Token::KeyWord(KeyWordType::Goto) => {
                let label = self.get_name()?;
                Ok(Statement::Goto { label })
            }
            Token::KeyWord(KeyWordType::Else) => {
                Err("Unexpected ELSE outside of IF block".to_string())
            }
            other => Err(format!("Unexpected command token {:?}", other))
        }
    }
    
    fn parse_let(&mut self) -> Result<Statement<'a>, String> {
        // get variable name
        let name = self.get_name()?;

        // check if = exist
        if self.next() != Some(Token::CmpOp(CmpOp::Equal)) {
            return Err(format!("Expected ="))
        }

        // get the value, which will be stored in hashmap
        let value = self.expr_bp(0)?;
        Ok(Statement::Assign { name, value })
    }

    fn parse_print(&mut self) -> Result<Statement<'a>, String> {
        // match if we need to print number or string
        match self.next() {
            // String 
            Some(Token::Literal(Literal::Text(text))) => Ok(Statement::PrintStr(text)),
            // Number or variable which store number
            Some(Token::Literal(Literal::Ident(name))) => Ok(Statement::PrintVar(name)),
            other => Err(format!("Expected string or variable after PRINT, found {:?}", other))
        }
    }

    fn parse_if(&mut self) -> Result<Statement<'a>, String> {
        // get left expression
        let left_value = self.expr_bp(0)?;
        
        // get operator (==, !=, <, >)
        let op_token = self.next();
        let cmp = match op_token {
            Some(Token::CmpOp(CmpOp::DoubleEqual)) => "=",
            Some(Token::CmpOp(CmpOp::NonEqual)) => "!",
            Some(Token::CmpOp(CmpOp::Less)) => "<",
            Some(Token::CmpOp(CmpOp::Greater)) => ">",
            Some(Token::CmpOp(CmpOp::LessEqual)) => "<=",
            Some(Token::CmpOp(CmpOp::GreaterEqual)) => ">=",
            other => return Err(format!("Expected == or !=, got {:?}", other)),
        };
        

        // get right expression
        let right_value = self.expr_bp(0)?;
        
        // check if keyword THEN exist
        if self.next() != Some(Token::KeyWord(KeyWordType::Then)) {
                return Err(format!("Expected block THEN"));
        }
       
        // Skip \n
        if let Some(Token::Newline) = self.peek() {
            self.next();
        }
        
        // Get the stop keywords to get the then_block before we meet ELSE
        let stop_keywords = vec![Token::KeyWord(KeyWordType::Else), Token::KeyWord(KeyWordType::Wend), Token::KeyWord(KeyWordType::End)];

        let then_block = self.parse_block(&stop_keywords)?;

        // Start parsing ELSE block
        let else_block = if let Some(Token::KeyWord(KeyWordType::Else)) = self.peek() {
            self.next(); // Consume Else

            if let Some(Token::Newline) = self.peek() {
                self.next();
            }

            let stop_keywords = vec![
                Token::KeyWord(KeyWordType::End),
                Token::KeyWord(KeyWordType::Wend),
                Token::EOF,
            ];

            self.parse_block(&stop_keywords)?
        } else {
            Vec::new()
        };

        
        if self.next() != Some(Token::KeyWord(KeyWordType::End)) {
            return Err("Expected END after IF block".to_string());
        }
        
        
        if let Some(Token::Newline) = self.peek() {
            self.next();
        }


        Ok(Statement::If { 
            left_value, 
            cmp, 
            right_value,
            then_block,
            else_block,
        })
    }

    fn parse_while(&mut self) -> Result<Statement<'a>, String> {
        // get left expression
        let left_value = self.expr_bp(0)?;

        // get operator (==, !=, <, >)
        let op_token = self.next();
        let cmp = match op_token {
            Some(Token::CmpOp(CmpOp::DoubleEqual)) => "=",
            Some(Token::CmpOp(CmpOp::NonEqual)) => "!",
            Some(Token::CmpOp(CmpOp::Less)) => "<",
            Some(Token::CmpOp(CmpOp::Greater)) => ">",
            Some(Token::CmpOp(CmpOp::LessEqual)) => "<=",
            Some(Token::CmpOp(CmpOp::GreaterEqual)) => ">=",
            other => return Err(format!("Expected == or !=, got {:?}", other)),
        };

        // get right expression
        let right_value = self.expr_bp(0)?;

        if self.next() != Some(Token::KeyWord(KeyWordType::Then)) {
            return Err(format!("Expected block THEN"));
        }

        if let Some(Token::Newline) = self.peek() {
            self.next();
        }

        Ok(Statement::While { 
            left_value, 
            cmp, 
            right_value, 
            end_idx: Cell::new(0) })
    }

    fn parse_random(&mut self) -> Result<Statement<'a>, String> {
        // get name of varibale
        let name = self.get_name()?;
        
        // get min and max to set a borders
        let min = self.get_num()?;
        let max = self.get_num()?;

        Ok (Statement::Random { name, min, max })
    }

    fn expr_bp(&mut self, min_bp: u8) -> Result<Expression<'a>, String> {
        let mut lhs = match self.next() {
            Some(Token::Literal(Literal::Number(num))) => Expression::Atom(num),
            Some(Token::OpType(OpType::LParen)) => {
                let lhs = self.expr_bp( 0)?;
                if self.next() != Some(Token::OpType(OpType::RParen)) {
                    return Err("Syntax Error: Expected matching ')'".to_string());
                }
                lhs
            }
            Some(Token::OpType(op_type)) => {
                match op_type {
                    OpType::Plus | OpType::Minus => {
                        let ((), r_bp) = self.prefix_bind_operator(op_type)?;
                        let rhs = self.expr_bp(r_bp)?;
                        Expression::Cons(op_type, vec![rhs])
                    }
                    _ => return Err(format!("Unexpected operator {:?}", op_type)),
                }
            }
            Some(Token::Literal(Literal::Ident(name))) => Expression::Variable(name),
            t => return Err(format!("Expected number, variable or prefix operator, but found {:?}", t)),
        }; 

        loop {
            let op = match self.peek() {
                Some(Token::OpType(op)) => *op,
                _ => break
            };

            if let Ok((left_power, ())) = self.postfix_bind_operator(op) {
                if left_power < min_bp {
                    break;
                }
                self.next();

                lhs = Expression::Cons(op, vec![lhs]);
                continue;
            }

            if let Ok((left_power, right_power)) = self.infix_bind_operator(op) {
                if left_power < min_bp {
                    break;
                }
                self.next();
                let rhs = self.expr_bp(right_power)?;
                lhs = Expression::Cons(op, vec![lhs, rhs]);
                continue;
            }
            break;
        }

        Ok(lhs)
    }
    fn prefix_bind_operator(&self, op_type: OpType) -> Result<((), u8), String> {
        match op_type {
            OpType::Plus | OpType::Minus => Ok(((), 5)),
            _ => Err(format!("Wrong prefix operator {:?}", op_type)),
        }
    }

    fn postfix_bind_operator(&self, op_type: OpType) -> Result<(u8, ()), String> {
        match op_type {
            OpType::Factorial => Ok((8, ())),
            _ => Err(format!("Wrong postfix operator {:?}", op_type)),
        }
    }

    fn infix_bind_operator(&self, op_type: OpType) -> Result<(u8, u8), String> {
        match op_type {
            OpType::Plus | OpType::Minus => Ok((1, 2)),
            OpType::Multiply | OpType::Divide | OpType::Mod => Ok((3, 4)),
            OpType::Power => Ok((7, 6)),
            _ => Err(format!("Wrong infix operator {:?}", op_type)),
        }
    }
}
