use std::iter::Peekable;

use crate::{
    AppIdentity, Arg, ArgParser, ArgValidator, ParsedArg, paragraph,
    tui::{self, RgbColor},
};

pub struct App {
    identity: AppIdentity,
    parser: ArgParser,
    parsed: ParsedArg,
    raw_args: Peekable<std::vec::IntoIter<String>>,
}

impl App {
    pub fn new(identity: AppIdentity) -> Self {
        Self {
            identity,
            parser: ArgParser::new(),
            parsed: ParsedArg::new(),
            raw_args: std::env::args().collect::<Vec<_>>().into_iter().peekable(),
        }
    }

    pub fn identity(&self) -> &AppIdentity {
        &self.identity
    }

    pub fn args(&self) -> &ParsedArg {
        &self.parsed
    }

    pub fn add_argument(&mut self, key: &str, arg: Arg) {
        self.parser.add_argument(key, arg);
    }

    pub fn add_positional_argument(&mut self, arg: Arg) {
        self.parser.add_positional_argument(arg);
        self.add_help_arguments();
    }
    pub fn add_help_arguments(&mut self) {
        self.parser.add_argument(
            "-h",
            Arg::new()
                .help("Show the help message for the application")
                .as_flag(),
        );
        self.parser.add_argument(
            "--help",
            Arg::new()
                .help("Show the help message for the application")
                .as_flag(),
        );
    }

    pub fn arg_len(&self) -> usize {
        self.parser.len()
    }

    pub fn print_help_text(&mut self) {
        let mut root_l = tui::Layout::new();

        root_l = root_l.append_child(
            tui::Layout::new()
                .style(
                    tui::DomStyle::new()
                        .fg(RgbColor::blue())
                        .effect(tui::TextEffect::Bold),
                )
                .append_child(paragraph!(
                    "{} v{}",
                    self.identity.name,
                    self.identity.version
                )),
        );

        if !self.identity.description.is_empty() {
            root_l = root_l.append_child(paragraph!("{}", &self.identity().description));
        }
        if let Some(author) = &self.identity.author {
            root_l = root_l.append_child(paragraph!("Written by : {}", author));
        }
        if let Some(license) = &self.identity.license {
            root_l = root_l.append_child(paragraph!("{}", license));
        }
        root_l = root_l.append_child(
            tui::Layout::new()
                .style(
                    tui::DomStyle::new()
                        .fg(tui::RgbColor::blue())
                        .effect(tui::TextEffect::Bold),
                )
                .append_child(match self.parser.len() {
                    1 => paragraph!(""),
                    _ => paragraph!("[ Arguments ]"),
                }),
        );

        let mut arg_l = tui::Layout::new();
        for (idx, tier) in self.parser.iter().enumerate() {
            if idx != 0 {
                arg_l = arg_l
                    .append_child(
                        tui::Layout::new()
                            .style(
                                tui::DomStyle::new()
                                    .fg(tui::RgbColor::bright_magenta())
                                    .effect(tui::TextEffect::Bold),
                            )
                            .append_child(paragraph!("arg{}", idx)),
                    )
                    .append_child(
                        tui::Layout::new()
                            .style(tui::DomStyle::new().indent(2))
                            .append_child(match ArgValidator::help(&tier.pos) {
                                Some(n) => n,
                                None => paragraph!("[  No Help ]"),
                            }),
                    );
            }
            if tier.is_empty() {
                arg_l = arg_l.append_child(paragraph!("[ No Keyword Arguments Defined ]"));
                continue;
            }
            arg_l = arg_l.append_child(
                tui::Layout::new()
                    .style(
                        tui::DomStyle::new()
                            .fg(tui::RgbColor::bright_magenta())
                            .effect(tui::TextEffect::Bold),
                    )
                    .append_child(paragraph!("[ Keyword Arguments ]")),
            );
            for (key, arg) in tier.params_iter() {
                arg_l = arg_l.append_child(
                    tui::Layout::new()
                        .append_child(
                            tui::Layout::new()
                                .style(tui::DomStyle::new().fg(tui::RgbColor::green()))
                                .append_child(paragraph!("{}", key)),
                        )
                        .append_child(
                            tui::Layout::new()
                                .style(tui::DomStyle::new().indent(2))
                                .append_child(match ArgValidator::help(arg) {
                                    Some(n) => n,
                                    None => paragraph!("<no-help>"),
                                }),
                        ),
                );
            }

            // let mut section = tui::Layout::new().style(style.clone());

            // /* Parametric Argument  */
            // section = section.append_child(paragraph!("arg{idx}:"));
            // let mut section_child = tui::Layout::new().style(style.clone().indent(2));
            // if let Some(node) = ArgValidator::help(&tier.pos) {
            //     section_child = section_child.append_child(node);
            // } else {
            //     section_child = section_child.append_child(paragraph!("<no-help>"));
            // }
            // section = section.append_child(section_child);

            // /* Keyword Arguments */
            // if tier.is_empty() {
            //     section = section.append_child(paragraph!("<no keyword arguments defined>"));
            // } else {
            //     section = section.append_child(paragraph!("Keyword Arguments:"));
            //     for (key, arg) in tier.params_iter() {
            //         /* Title  */
            //         let mut entry = tui::Layout::new()
            //             .style(tui::DomStyle::new().fg(tui::RgbColor::blue()).indent(2));
            //         entry = entry.append_child(paragraph!("{}", key));

            //         /* Children  */
            //         let mut entry_child = tui::Layout::new().style(style.clone().indent(2));
            //         if let Some(node) = ArgValidator::help(arg) {
            //             entry_child = entry_child.append_child(node);
            //         } else {
            //             entry_child = entry_child.append_child(paragraph!("<no-help>"));
            //         }

            //         /* Add it back */
            //         entry = entry.append_child(entry_child);
            //         section = section.append_child(tui::VStack(entry));
            //     }
            // }
            // layout = layout.append_child(tui::VStack(section));
            // layout = layout.append_child(paragraph!(""));
        }
        root_l = root_l.append_child(arg_l);
        println!("{}", &tui::VStack(root_l));
    }

    pub fn parse_args(&mut self, auto_help: bool) -> &ParsedArg {
        let res = self
            .parser
            .incremental_parse(&mut self.parsed, &mut self.raw_args);
        if auto_help && (self.parsed.count("-h") + self.parsed.count("--help") > 0) {
            self.print_help_text();
            std::process::exit(0);
        }
        match res {
            Ok(_) => &self.parsed,
            Err(err) => {
                eprintln!(
                    "{}",
                    tui::VStack(
                        tui::Layout::default()
                            .append_child(paragraph!("{}", err))
                            .style(tui::DomStyle::new().fg(tui::RgbColor::bright_yellow())),
                    )
                );
                std::process::exit(1);
            }
        }
    }
}
