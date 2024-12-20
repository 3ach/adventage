use proc_macro::TokenStream;
use quote::quote;
use quote::ToTokens;
use quote::format_ident;
use syn::{Lit, Expr, Token, Result};
use syn::punctuated::Punctuated;
use syn::parse::Parser;
use syn::parse::Parse;
use syn::parse::ParseStream;

static mut DEMOS: u32 = 0;

#[proc_macro]
pub fn day(attrs: TokenStream) -> TokenStream {
    let parser = Punctuated::<Expr, Token![,]>::parse_terminated;
	let args = parser.parse(attrs).unwrap().iter()
		.filter_map(|x| match x {
			Expr::Lit(el) => Some(el.lit.clone()),
			_ => None
		})
		.filter_map(|lit| match lit {
			Lit::Int(li) => Some(li.base10_parse::<u32>().unwrap()),
			_ => None,
		}).collect::<Vec<u32>>();

	let year = args.iter().max().unwrap();
	let day = args.iter().min().unwrap(); 

    let main: syn::ItemFn = syn::parse(quote! {
        fn main() -> Result<(), ()> {
            let fetch_start = std::time::Instant::now();
            let input = adventage::fetch_day(#year, #day);
            let fetch_runtime = adventage::format_runtime(fetch_start.elapsed());
            println!("Fetching the input took {fetch_runtime}");

            let parsed_start = std::time::Instant::now();
            let parsed = parse(&input);
            let parse_runtime = adventage::format_runtime(parsed_start.elapsed());
            println!("Parsing the input took {parse_runtime}");

            let part1_start = std::time::Instant::now();
            let answer1 = part1(&parsed);
            let part1_runtime = adventage::format_runtime(part1_start.elapsed());
            println!("Answer 1: {answer1}, took {part1_runtime}");

            let part2_start = std::time::Instant::now();
            let answer2 = part2(&parsed);
            let part2_runtime = adventage::format_runtime(part2_start.elapsed());
            println!("Answer 2: {answer2}, took {part2_runtime}");
            Ok(())
        }
    }.into()).unwrap();

    main.to_token_stream().into()
}

#[derive(Debug)]
enum Answer {
    StringAnswer(String),
    NumberAnswer(String),
}

impl Parse for Answer {
    fn parse(input: ParseStream) -> Result<Self> {
        let literal: Lit = input.parse()?;

        match literal {
            Lit::Str(ls) => Ok(Answer::StringAnswer(ls.value())),
            Lit::Int(li) => Ok(Answer::NumberAnswer(li.base10_digits().to_string())),
            _ => panic!(),
        }
    }
}

#[proc_macro]
pub fn part1demo(attrs: TokenStream) -> TokenStream {
    let parser = Punctuated::<Answer, Token![,]>::parse_terminated;
    let result = parser.parse(attrs).unwrap();
    let Answer::StringAnswer(input) = &result[0] else { panic!() };
    let num = unsafe { DEMOS += 1; DEMOS };
    let name = format_ident!("test_part1_{}", num);

    let assertion = match &result[1] {
        Answer::StringAnswer(answer) => quote! { assert_eq!(answer, #answer); },
        Answer::NumberAnswer(answer) => quote! { assert_eq!(answer, #answer.parse().unwrap()); }
    };

    let test: syn::ItemFn = syn::parse(quote! {
        #[test]
        fn #name() {
            let parsed_input = parse(#input);
            let answer = part1(&parsed_input);

            println!("Part 1 example:");
            println!("{}", #input);
            println!("Part 1 answer: {}", answer);
            println!("--");

            #assertion
        }
    }.into()).unwrap();

    test.to_token_stream().into()
}

#[proc_macro]
pub fn part2demo(attrs: TokenStream) -> TokenStream {
    let parser = Punctuated::<Answer, Token![,]>::parse_terminated;
    let result = parser.parse(attrs).unwrap();
    let Answer::StringAnswer(input) = &result[0] else { panic!() };

    let assertion = match &result[1] {
        Answer::StringAnswer(answer) => quote! { assert_eq!(answer, #answer); },
        Answer::NumberAnswer(answer) => quote! { assert_eq!(answer, #answer.parse().unwrap()); }
    };
 
    let num = unsafe { DEMOS += 1; DEMOS };
    let name = format_ident!("test_part2_{}", num);

    let test: syn::ItemFn = syn::parse(quote! {
        #[test]
        fn #name() {
            let parsed_input = parse(#input);
            let answer = part2(&parsed_input);

            println!("Part 2 example:");
            println!("{}", #input);
            println!("Part 2 answer: {}", answer);
            println!("--");

            #assertion
        }
    }.into()).unwrap();

    test.to_token_stream().into()
}
