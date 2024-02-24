use {
    proc_macro::TokenStream,
    quote::quote,
    std::fmt::{Display, Formatter},
    syn::{parse_macro_input, Error, Expr, ExprBlock, Ident, ItemFn, Stmt},
};

#[derive(Debug, Copy, Clone)]
enum SectionType {
    Given,
    When,
    Then,
    AndGiven,
    AndWhen,
    AndThen,
}

impl SectionType {
    fn all() -> [Self; 6] {
        [
            Self::Given,
            Self::When,
            Self::Then,
            Self::AndGiven,
            Self::AndWhen,
            Self::AndThen,
        ]
    }

    fn prefix(&self) -> &str {
        match self {
            SectionType::Given => "given_",
            SectionType::When => "when_",
            SectionType::Then => "then_",
            SectionType::AndGiven => "and_given_",
            SectionType::AndWhen => "and_when_",
            SectionType::AndThen => "and_then_",
        }
    }
}

#[derive(Debug, Clone)]
struct Section {
    section_type: SectionType,
    ident: Ident,
    block: ExprBlock,
}

impl Section {
    fn statements(&self) -> Vec<Stmt> {
        self.block.block.stmts.clone()
    }
}

#[derive(Debug, Clone)]
struct Context {
    function: ItemFn,
    sections: Vec<Section>,
    body: Vec<Stmt>,
}

impl Context {
    fn append_section(&self, section: Section) -> Self {
        let mut context = self.clone();
        context.sections.push(section);
        context
    }

    fn remaining_statements(&self) -> Vec<Stmt> {
        self.sections
            .last()
            .map(Section::statements)
            .unwrap_or_default()
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

fn get_section(block: &ExprBlock) -> Option<Section> {
    if let Some(label) = &block.label {
        for section_type in SectionType::all() {
            let ident = label.name.ident.to_string();

            if ident.starts_with(section_type.prefix()) {
                let (_, description) = ident.split_once(section_type.prefix()).unwrap();
                let ident = Ident::new(description, label.name.ident.span());

                return Some(Section {
                    section_type,
                    ident,
                    block: block.clone(),
                });
            }
        }
    }

    None
}

fn given(mut context: Context) -> Result<proc_macro2::TokenStream, Error> {
    let mut whens = vec![];
    let mut givens = vec![];
    let mut thens = vec![];

    for statement in context.remaining_statements() {
        match statement {
            Stmt::Expr(Expr::Block(ref block)) => {
                if let Some(section) = get_section(block) {
                    let context = context.append_section(section.clone());

                    match section.section_type {
                        SectionType::When => {
                            whens.push(when(context)?);
                        }
                        SectionType::Then => {
                            thens.push(then(context)?);
                        }
                        SectionType::AndGiven => {
                            givens.push(given(context)?);
                        }
                        _ => {
                            return Err(Error::new(
                                section.ident.span(),
                                "Only \"when\", \"then\", or \"and_given\" are allowed inside \"given\" section",
                            ));
                        }
                    }
                } else {
                    context.body.push(statement);
                }
            }
            statement => context.body.push(statement),
        }
    }

    let ident = context.sections.last().unwrap().ident.clone();
    Ok(quote! {
        mod #ident {
            use super::*;
            mod and {
                use super::*;
                #(#givens)*
            }
            mod when {
                use super::*;
                #(#whens)*
            }
            mod then {
                use super::*;
                #(#thens)*
            }
        }
    })
}

fn when(mut context: Context) -> Result<proc_macro2::TokenStream, Error> {
    let mut thens = vec![];
    let mut whens = vec![];

    for statement in context.remaining_statements() {
        match statement {
            Stmt::Expr(Expr::Block(ref block)) => {
                if let Some(section) = get_section(block) {
                    let inner_context = context.append_section(section.clone());

                    match section.section_type {
                        SectionType::Then => {
                            thens.push(then(inner_context)?);
                        }
                        SectionType::AndWhen => {
                            whens.push(when(inner_context)?);
                        }
                        _ => {
                            return Err(Error::new(
                                section.ident.span(),
                                "Only \"then\" or \"and_when\" are allowed inside \"when\" section",
                            ));
                        }
                    }
                } else {
                    context.body.push(statement);
                }
            }
            statement => context.body.push(statement),
        }
    }

    let ident = context.sections.last().unwrap().ident.clone();
    Ok(quote! {
        mod #ident {
            use super::*;
            mod and {
                use super::*;
                #(#whens)*
            }
            mod then {
                use super::*;
                #(#thens)*
            }
        }
    })
}

fn then(mut context: Context) -> Result<proc_macro2::TokenStream, Error> {
    let mut thens = vec![];

    for statement in context.remaining_statements() {
        match statement {
            Stmt::Expr(Expr::Block(ref block)) => {
                if let Some(section) = get_section(block) {
                    let context = context.append_section(section.clone());

                    match section.section_type {
                        SectionType::AndThen => {
                            thens.push(then(context)?);
                        }
                        _ => {
                            return Err(Error::new(
                                section.ident.span(),
                                "Only \"and_then\" is allowed inside \"then\" section",
                            ));
                        }
                    }
                } else {
                    context.body.push(statement);
                }
            }
            statement => {
                context.body.push(statement);
            }
        }
    }

    let scenario_description = context.sections.iter().fold(
        format!(
            "Scenario: {}\n",
            ident_to_string(&context.function.sig.ident)
        ),
        |str, section| format!("{}{}\n", str, section),
    );

    let ident = context.sections.last().unwrap().ident.clone();
    let attributes = context.function.attrs;
    let asyncness = context.function.sig.asyncness;
    let test_body = context.body.clone();

    Ok(quote! {
        #(#attributes)*
        #[allow(unused_variables, unused_mut, unused_labels)]
        #asyncness fn #ident() {
            println!("\n{}", #scenario_description);
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
    let function = parse_macro_input!(item as ItemFn);

    let mut context = Context {
        function: function.clone(),
        sections: vec![],
        body: vec![],
    };

    let mut givens = vec![];

    for statement in function.block.stmts {
        match statement {
            Stmt::Expr(Expr::Block(ref block)) => match get_section(block) {
                Some(section) => {
                    let context = context.append_section(section.clone());

                    match section.section_type {
                        SectionType::Given => {
                            match given(context) {
                                Ok(given) => givens.push(given),
                                Err(err) => return err.to_compile_error().into(),
                            };
                        }
                        _ => {
                            return Error::new(
                                section.ident.span(),
                                "Only \"given\" is allowed inside \"scenario\" section",
                            )
                            .to_compile_error()
                            .into();
                        }
                    }
                }
                None => {
                    context.body.push(statement.clone());
                }
            },
            statement => context.body.push(statement),
        }
    }

    let scenario = context.function.sig.ident;
    let givens = if !givens.is_empty() {
        Some(quote! {
            mod given {
                use super::*;
                #(#givens)*
            }
        })
    } else {
        None
    };

    quote!(
        mod #scenario {
            use super::*;
            #givens
        }
    )
    .into()
}
