pub mod buffered;
pub mod command_line;
mod regex;

use tokio::sync::OnceCell;

use crate::{command_line_args::CommandLineArgs, common::OwnedCommandAndArgs};

use self::{
    buffered::BufferedInputLineParser, command_line::CommandLineArgsParser, regex::RegexProcessor,
};

struct ShellCommandAndArgs(Option<Vec<String>>);

impl ShellCommandAndArgs {
    fn new(command_line_args: &CommandLineArgs) -> Self {
        Self(if command_line_args.shell {
            Some(vec![
                command_line_args.shell_path.clone(),
                Self::shell_argument().to_owned(),
            ])
        } else {
            None
        })
    }

    fn shell_argument() -> &'static str {
        if cfg!(target_os = "windows") && cfg!(feature = "win_cmd_shell") {
            "/c"
        } else {
            "-c"
        }
    }
}

fn build_owned_command_and_args(
    shell_command_and_args: &ShellCommandAndArgs,
    command_and_args: Vec<String>,
) -> Option<OwnedCommandAndArgs> {
    match &shell_command_and_args.0 {
        None => OwnedCommandAndArgs::try_from(command_and_args).ok(),
        Some(shell_command_and_args) => {
            let mut result = Vec::with_capacity(shell_command_and_args.len() + 1);

            result.extend(shell_command_and_args.iter().cloned());
            result.push(command_and_args.join(" "));

            OwnedCommandAndArgs::try_from(result).ok()
        }
    }
}

pub struct Parser {
    buffered_input_line_parser: OnceCell<BufferedInputLineParser>,
    regex_processor: RegexProcessor,
    command_line_args: &'static CommandLineArgs,
}

impl Parser {
    pub fn new(command_line_args: &'static CommandLineArgs) -> anyhow::Result<Self> {
        let regex_processor = RegexProcessor::new(command_line_args)?;
        Ok(Self {
            buffered_input_line_parser: OnceCell::new(),
            regex_processor,
            command_line_args,
        })
    }

    pub async fn buffered_input_line_parser(&self) -> &BufferedInputLineParser {
        self.buffered_input_line_parser
            .get_or_init(|| async move {
                BufferedInputLineParser::new(self.command_line_args, self.regex_processor.clone())
            })
            .await
    }

    pub fn command_line_args_parser(&self) -> CommandLineArgsParser {
        CommandLineArgsParser::new(self.command_line_args, self.regex_processor.clone())
    }
}
