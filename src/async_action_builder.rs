use async_trait::async_trait;

use crate::tui;
use crate::{App, Arg, ArgOptionValidator, paragraph};

#[async_trait]
pub trait AsyncActionHandler: Send {
    async fn run(&mut self, app: &mut App);
}

struct AppAction {
    name: String,
    help_text: String,
    handler: Box<dyn AsyncActionHandler>,
}

pub struct AsyncActionBuilder<'a> {
    app: &'a mut App,
    help_text: Option<String>,
    actions: Vec<AppAction>,
}

impl<'a> AsyncActionBuilder<'a> {
    pub fn new(app: &'a mut App, help_text: Option<String>) -> Self {
        Self {
            app,
            help_text,
            actions: Vec::new(),
        }
    }

    pub fn add_action(
        mut self,
        name: impl Into<String>,
        help_text: impl Into<String>,
        handler: impl AsyncActionHandler + 'static,
    ) -> Self {
        let name = name.into();
        if let Some(action) = self.actions.iter_mut().find(|action| action.name == name) {
            action.help_text = help_text.into();
            action.handler = Box::new(handler);
        } else {
            self.actions.push(AppAction {
                name,
                help_text: help_text.into(),
                handler: Box::new(handler),
            });
        }
        self
    }

    pub async fn run(self) {
        if self.actions.is_empty() {
            return;
        }

        let AsyncActionBuilder {
            app,
            help_text,
            actions,
        } = self;

        let mut argument = Arg::new();
        if let Some(help) = help_text {
            argument = argument.help(help);
        }
        let mut options = ArgOptionValidator::new();
        for action in &actions {
            options = options.option(action.name.clone(), Some(action.help_text.clone()));
        }
        argument = argument.validate(options).required();

        app.add_positional_argument(argument);
        let action_index = app.arg_len() - 1;

        app.parse_args(false);

        let have_help = app.args().contains("-h") || app.args().contains("--help");
        if app.args().len() <= action_index && !have_help {
            eprintln!(
                "{}",
                tui::VStack(
                    tui::Layout::default()
                        .append_child(paragraph!("arg{}: expected action name", action_index))
                        .style(tui::DomStyle::new().fg(tui::RgbColor::bright_yellow())),
                )
            );
            std::process::exit(1)
        } else if app.args().len() <= action_index && have_help {
            app.print_help_text();
            std::process::exit(0);
        }

        let action_name = app.args().arg().to_string();
        match actions
            .into_iter()
            .find(|action| action.name == action_name)
        {
            Some(mut action) => action.handler.run(app).await,
            None => {
                eprintln!(
                    "{}",
                    &tui::VStack(
                        tui::Layout::default()
                            .append_child(paragraph!("Unknown action '{}'", action_name))
                            .style(tui::DomStyle::new().fg(tui::RgbColor::bright_yellow())),
                    )
                );
                std::process::exit(1)
            }
        }
    }
}
