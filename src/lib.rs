use proc_macro::TokenStream;
use quote::quote;
use std::fmt::{Display, Formatter};
use syn::{
    parse_macro_input, parse_quote, spanned::Spanned, Attribute, Block, Error, Expr,
    Ident, ItemFn, Stmt,
};

#[derive(Debug, Clone)]
enum SectionType {
    Given,
    When,
    Then,
    AndGiven,
    AndWhen,
    AndThen,
}

#[derive(Debug, Clone)]
struct Section {
    section_type: SectionType,
    ident: Ident,
    attribute: Attribute,
}

#[derive(Debug, Clone)]
struct Context {
    function: ItemFn,
    sections: Vec<Section>,
    test_body: Vec<Stmt>,
}

impl Context {
    pub fn with_section(&self, section: Section) -> Self {
        let mut context = self.clone();
        context.sections.push(section);
        context
    }
}

fn ident_to_string(ident: &Ident) -> String {
    ident.to_string().replace('_', " ")
}

impl Display for Section {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ident = ident_to_string(&self.ident);

        match self.section_type {
            SectionType::Given => write!(f, "   Given: {}", ident),
            SectionType::When => write!(f, "    When: {}", ident),
            SectionType::Then => write!(f, "    Then: {}", ident),
            SectionType::AndGiven => write!(f, "     and: {}", ident),
            SectionType::AndWhen => write!(f, "     and: {}", ident),
            SectionType::AndThen => write!(f, "     and: {}", ident),
        }
    }
}

fn get_section(attributes: &Vec<Attribute>) -> Option<Result<Section, Error>> {
    for attribute in attributes {
        let section_type = if attribute.path.is_ident("given") {
            Some(SectionType::Given)
        } else if attribute.path.is_ident("when") {
            Some(SectionType::When)
        } else if attribute.path.is_ident("then") {
            Some(SectionType::Then)
        } else if attribute.path.is_ident("and_given") {
            Some(SectionType::AndGiven)
        } else if attribute.path.is_ident("and_when") {
            Some(SectionType::AndWhen)
        } else if attribute.path.is_ident("and_then") {
            Some(SectionType::AndThen)
        } else {
            None
        };

        if let Some(section_type) = section_type {
            return match attribute.parse_args::<Ident>() {
                Ok(ident) => Some(Ok(Section {
                    section_type,
                    ident,
                    attribute: attribute.clone(),
                })),
                Err(e) => Some(Err(e)),
            };
        }
    }

    None
}

fn given(mut context: Context, block: Block) -> Result<proc_macro2::TokenStream, Error> {
    let mut whens = vec![];
    let mut givens = vec![];

    for statement in block.stmts {
        match statement {
            Stmt::Expr(Expr::Block(ref block)) => {
                let section = get_section(&block.attrs);

                match section {
                    Some(section) => {
                        let section = section?;
                        let block = block.block.clone();

                        match section.section_type {
                            SectionType::When => {
                                whens.push(when(context.with_section(section), block)?);
                            }
                            SectionType::AndGiven => {
                                givens.push(given(context.with_section(section), block)?);
                            }
                            _ => {
                                return Err(Error::new(
                                    section.attribute.path.span(),
                                    "Only \"when\" or \"and_given\" are allowed inside \"given\" section",
                                ));
                            }
                        }
                    }
                    None => {
                        context.test_body.push(statement);
                    }
                }
            }
            statement => context.test_body.push(statement),
        }
    }

    let ident = context.sections.last().unwrap().ident.clone();

    Ok(quote! {
        mod #ident {
            use super::*;

            mod when {
                use super::*;
                #(#whens)*
            }

            mod and {
                use super::*;
                #(#givens)*
            }
        }
    })
}

fn when(mut context: Context, block: Block) -> Result<proc_macro2::TokenStream, Error> {
    let mut thens = vec![];
    let mut whens = vec![];

    for statement in block.stmts {
        match statement {
            Stmt::Expr(Expr::Block(ref block)) => match get_section(&block.attrs) {
                Some(section) => {
                    let section = section?;
                    let block = block.block.clone();

                    match section.section_type {
                        SectionType::Then => {
                            thens.push(then(context.with_section(section), block)?);
                        }
                        SectionType::AndWhen => {
                            whens.push(when(context.with_section(section), block)?);
                        }
                        _ => {
                            return Err(Error::new(
                                section.attribute.path.span(),
                                "Only \"then\" or \"and_when\" are allowed inside \"when\" section",
                            ));
                        }
                    }
                }
                None => {
                    context.test_body.push(statement);
                }
            },
            Stmt::Semi(Expr::Macro(ref mac, ), _) => {
                match get_section(&mac.attrs) {
                    Some(section) => {
                        let section = section?;

                        if ! matches!(section.section_type, SectionType::Then) {
                            return Err(Error::new(
                                section.attribute.path.span(),
                                "Only \"then\" or \"and_when\" are allowed inside \"when\" section",
                            ));
                        }

                        let mac = mac.mac.clone();
                        let block = parse_quote! {
                            { #mac }
                        };

                        thens.push(then(context.with_section(section), block)?);
                    }
                    None => {
                        context.test_body.push(statement);
                    }
                }
            }
            statement => context.test_body.push(statement),
        }
    }

    let ident = context.sections.last().unwrap().ident.clone();

    Ok(quote! {
        mod #ident {
            use super::*;

            mod then {
                use super::*;
                #(#thens)*
            }

            mod and {
                use super::*;
                #(#whens)*
            }
        }
    })
}

fn then(mut context: Context, block: Block) -> Result<proc_macro2::TokenStream, Error> {
    let mut thens = vec![];

    for statement in block.stmts {
        match statement {
            Stmt::Expr(Expr::Block(ref block)) => match get_section(&block.attrs) {
                Some(section) => {
                    let section = section?;
                    let block = block.block.clone();

                    match section.section_type {
                        SectionType::AndThen => {
                            thens.push(then(context.with_section(section), block)?);
                        }
                        _ => {
                            return Err(Error::new(
                                section.attribute.path.span(),
                                "Only \"and_then\" is allowed inside \"then\" section",
                            ));
                        }
                    }
                }
                None => {
                    context.test_body.push(statement);
                }
            },
            Stmt::Semi(Expr::Macro(ref mac, ), _) => {
                match get_section(&mac.attrs) {
                    Some(section) => {
                        let section = section?;

                        if ! matches!(section.section_type, SectionType::AndThen) {
                            return Err(Error::new(
                                section.attribute.path.span(),
                                "Only \"and_then\" is allowed inside \"then\" section",
                            ));
                        }

                        let mac = mac.mac.clone();
                        let block = parse_quote! {
                            { #mac }
                        };

                        thens.push(then(context.with_section(section), block)?);
                    }
                    None => {
                        context.test_body.push(statement);
                    }
                }
            }
            statement => {
                context.test_body.push(statement);
            }
        }
    }

    let ident = context.sections.last().unwrap().ident.clone();
    let attributes = context.function.attrs;
    let asyncness = context.function.sig.asyncness;
    let test_body = context.test_body.clone();

    let given_when_then = context.sections.into_iter().fold(
        format!(
            "Scenario: {}\n",
            ident_to_string(&context.function.sig.ident)
        ),
        |str, section| format!("{}{}\n", str, section),
    );

    Ok(quote! {
        #(#attributes)*
        #asyncness fn #ident() {
            println!("\n{}", #given_when_then);
            #(#test_body)*
        }

        mod #ident {
            use super::*;

            mod and {
                use super::*;
                #(#thens)*
            }
        }
    })
}

#[proc_macro_attribute]
pub fn scenario(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut function = parse_macro_input!(item as ItemFn);

    if function.attrs.is_empty() {
        function.attrs.push(parse_quote! {
            #[test]
        });
    }

    let mut context = Context {
        function: function.clone(),
        sections: vec![],
        test_body: vec![],
    };

    let mut givens = vec![];

    for statement in function.block.stmts {
        match statement {
            Stmt::Expr(Expr::Block(ref block)) => match get_section(&block.attrs) {
                Some(Ok(section)) => match section.section_type {
                    SectionType::Given => {
                        match given(context.with_section(section), block.block.clone()) {
                            Ok(given) => givens.push(given),
                            Err(err) => return err.to_compile_error().into(),
                        };
                    }
                    _ => {
                        return Error::new(
                            section.attribute.path.span(),
                            "Only \"given\" is allowed inside \"scenario\" section",
                        )
                        .to_compile_error()
                        .into();
                    }
                },
                Some(Err(error)) => {
                    return error.to_compile_error().into();
                }
                None => {
                    context.test_body.push(statement.clone());
                }
            },
            statement => context.test_body.push(statement),
        }
    }

    let scenario = context.function.sig.ident;

    quote!(
        mod #scenario {
            use super::*;

            mod given {
                use super::*;
                #(#givens)*
            }
        }
    )
    .into()
}
