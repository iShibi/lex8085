use logos::{Lexer, Logos};

fn clean_label(lex: &mut Lexer<Token>) -> Option<String> {
	let slice = lex.slice().to_uppercase();
	let label = &slice[..slice.len() - 1];
	Some(label.to_string())
}

fn clean_comment(lex: &mut Lexer<Token>) -> Option<String> {
	let comment: String = lex.slice().strip_prefix(";")?.split_whitespace().collect::<Vec<&str>>().join(" ");
	Some(comment)
}

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
	#[regex("(?i)MOV|MVI|LXI|LDA|STA|LHLD|SHLD|LDAX|STAX|XCHG|ADD|ADC|SUB|SBB|ANA|XRA|ORA|CMP|ADI|ACI|SUI|SBI|ANI|XRI|ORI|CPI|RLC|RRC|RAL|RAR|CMA|CMC|STC|HLT|NOP|DCR|INR|DAD|DAA|JMP|CALL|RET|JNZ|JZ|JNC|JC|JPO|JPE|JP|JM|CNZ|CZ|CNC|CC|CPO|CPE|CP|CM|RIM|SIM|IN|OUT|PUSH|POP|XTHL|SPHL|PCHL", |lex| lex.slice().to_owned().to_uppercase())]
	Opcode(String),

	#[regex("(?i)A|B|C|D|E|H|L|[0-9A-F]{1,4}H|[a-zA-Z]+", |lex| lex.slice().to_owned().to_uppercase())]
	Operand(String),

	#[regex(r";.*", clean_comment)]
	Comment(String),

	#[regex("(?i)ORG|DB|DW|DS|EQU|END", |lex| lex.slice().to_owned().to_uppercase())]
	Directive(String),

	#[regex("(?i)[a-zA-Z]+:", clean_label)]
	Label(String),

	#[regex(",")]
	Comma,
}

pub fn create_tokens(str: String) -> Vec<Token> {
	let tokens: Vec<_> = Token::lexer(str.as_str()).filter_map(|op| op.ok()).collect();
	tokens
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
		let str = r#"
		ORG 0000H   ; Start the program at address 0000H

    MVI A, 42H  ; Load hexadecimal value 42H into the accumulator
    STA 2000H   ; Store the value in the accumulator into memory location 2000H

    HLT         ; Halt the program

		END         ; End of the program
		"#;

		let tokens = create_tokens(str.to_string());
		println!("{:#?}", tokens);
	}
}
