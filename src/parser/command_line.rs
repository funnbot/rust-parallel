use itertools::Itertools;

use crate::{
    command_line_args::{CommandLineArgs, COMMANDS_FROM_ARGS_SEPARATOR},
    common::OwnedCommandAndArgs,
};

#[derive(Debug)]
struct ArgumentGroups {
    first_command_and_args: Vec<String>,
    argument_groups: Vec<Vec<String>>,
}

pub struct CommandLineArgsParser {
    argument_groups: ArgumentGroups,
    shell_command_and_args: Option<Vec<String>>,
}

impl CommandLineArgsParser {
    pub fn new(command_line_args: &CommandLineArgs) -> Self {
        let argument_groups = Self::build_argument_groups(command_line_args);

        let shell_command_and_args = if command_line_args.shell {
            Some(vec![command_line_args.shell_path.clone(), "-c".to_owned()])
        } else {
            None
        };

        Self {
            argument_groups,
            shell_command_and_args,
        }
    }

    fn build_argument_groups(command_line_args: &CommandLineArgs) -> ArgumentGroups {
        let command_and_initial_arguments = &command_line_args.command_and_initial_arguments;

        let mut argument_groups = Vec::with_capacity(command_and_initial_arguments.len());

        let mut first = true;

        let mut first_command_and_args = vec![];

        for (separator, group) in &command_and_initial_arguments
            .iter()
            .group_by(|arg| *arg == COMMANDS_FROM_ARGS_SEPARATOR)
        {
            let group_vec = group.cloned().collect();

            if first {
                if !separator {
                    first_command_and_args = group_vec;
                }
                first = false;
            } else if !separator {
                argument_groups.push(group_vec);
            }
        }

        ArgumentGroups {
            first_command_and_args,
            argument_groups,
        }
    }

    pub fn parse_command_line_args(self) -> Vec<OwnedCommandAndArgs> {
        let ArgumentGroups {
            first_command_and_args,
            argument_groups,
        } = self.argument_groups;

        argument_groups
            .into_iter()
            .multi_cartesian_product()
            .filter_map(|current_args| match &self.shell_command_and_args {
                None => {
                    let cmd_and_args = [first_command_and_args.clone(), current_args].concat();
                    OwnedCommandAndArgs::try_from(cmd_and_args).ok()
                }
                Some(shell_command_and_args) => {
                    let merged_args = [first_command_and_args.clone(), current_args]
                        .concat()
                        .join(" ");
                    let merged_args = vec![merged_args];
                    let cmd_and_args = [shell_command_and_args.clone(), merged_args].concat();
                    OwnedCommandAndArgs::try_from(cmd_and_args).ok()
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::{default::Default, path::PathBuf};

    #[test]
    fn test_parse_command_line_args_with_intial_command() {
        let command_line_args = CommandLineArgs {
            shell: false,
            command_and_initial_arguments: vec![
                "echo", "-n", ":::", "A", "B", ":::", "C", "D", "E",
            ]
            .into_iter()
            .map_into()
            .collect(),
            ..Default::default()
        };

        let parser = CommandLineArgsParser::new(&command_line_args);

        let result = parser.parse_command_line_args();

        assert_eq!(
            result,
            vec![
                OwnedCommandAndArgs {
                    command_path: PathBuf::from("echo"),
                    args: vec!["-n", "A", "C"].into_iter().map_into().collect(),
                },
                OwnedCommandAndArgs {
                    command_path: PathBuf::from("echo"),
                    args: vec!["-n", "A", "D"].into_iter().map_into().collect(),
                },
                OwnedCommandAndArgs {
                    command_path: PathBuf::from("echo"),
                    args: vec!["-n", "A", "E"].into_iter().map_into().collect(),
                },
                OwnedCommandAndArgs {
                    command_path: PathBuf::from("echo"),
                    args: vec!["-n", "B", "C"].into_iter().map_into().collect(),
                },
                OwnedCommandAndArgs {
                    command_path: PathBuf::from("echo"),
                    args: vec!["-n", "B", "D"].into_iter().map_into().collect(),
                },
                OwnedCommandAndArgs {
                    command_path: PathBuf::from("echo"),
                    args: vec!["-n", "B", "E"].into_iter().map_into().collect(),
                },
            ]
        );
    }

    #[test]
    fn test_parse_command_line_args_no_intial_command() {
        let command_line_args = CommandLineArgs {
            shell: false,
            command_and_initial_arguments: vec![
                ":::", "echo", "say", ":::", "arg1", "arg2", "arg3",
            ]
            .into_iter()
            .map_into()
            .collect(),
            ..Default::default()
        };

        let parser = CommandLineArgsParser::new(&command_line_args);

        let result = parser.parse_command_line_args();

        assert_eq!(
            result,
            vec![
                OwnedCommandAndArgs {
                    command_path: PathBuf::from("echo"),
                    args: vec!["arg1"].into_iter().map_into().collect(),
                },
                OwnedCommandAndArgs {
                    command_path: PathBuf::from("echo"),
                    args: vec!["arg2"].into_iter().map_into().collect(),
                },
                OwnedCommandAndArgs {
                    command_path: PathBuf::from("echo"),
                    args: vec!["arg3"].into_iter().map_into().collect(),
                },
                OwnedCommandAndArgs {
                    command_path: PathBuf::from("say"),
                    args: vec!["arg1"].into_iter().map_into().collect(),
                },
                OwnedCommandAndArgs {
                    command_path: PathBuf::from("say"),
                    args: vec!["arg2"].into_iter().map_into().collect(),
                },
                OwnedCommandAndArgs {
                    command_path: PathBuf::from("say"),
                    args: vec!["arg3"].into_iter().map_into().collect(),
                },
            ]
        );
    }

    #[test]
    fn test_parse_command_line_args_empty() {
        let command_line_args = CommandLineArgs {
            shell: false,
            command_and_initial_arguments: vec![],
            ..Default::default()
        };

        let parser = CommandLineArgsParser::new(&command_line_args);

        let result = parser.parse_command_line_args();

        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_parse_command_line_args_invalid() {
        let command_line_args = CommandLineArgs {
            shell: false,
            command_and_initial_arguments: vec![":::", ":::"].into_iter().map_into().collect(),
            ..Default::default()
        };

        let parser = CommandLineArgsParser::new(&command_line_args);

        let result = parser.parse_command_line_args();

        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_parse_command_line_args_shell_mode() {
        let command_line_args = CommandLineArgs {
            shell: true,
            command_and_initial_arguments: vec![
                "echo", "-n", ":::", "A", "B", ":::", "C", "D", "E",
            ]
            .into_iter()
            .map_into()
            .collect(),
            shell_path: "/bin/bash".to_owned(),
            ..Default::default()
        };

        let parser = CommandLineArgsParser::new(&command_line_args);

        let result = parser.parse_command_line_args();

        assert_eq!(
            result,
            vec![
                OwnedCommandAndArgs {
                    command_path: PathBuf::from("/bin/bash"),
                    args: vec!["-c", "echo -n A C"].into_iter().map_into().collect(),
                },
                OwnedCommandAndArgs {
                    command_path: PathBuf::from("/bin/bash"),
                    args: vec!["-c", "echo -n A D"].into_iter().map_into().collect(),
                },
                OwnedCommandAndArgs {
                    command_path: PathBuf::from("/bin/bash"),
                    args: vec!["-c", "echo -n A E"].into_iter().map_into().collect(),
                },
                OwnedCommandAndArgs {
                    command_path: PathBuf::from("/bin/bash"),
                    args: vec!["-c", "echo -n B C"].into_iter().map_into().collect(),
                },
                OwnedCommandAndArgs {
                    command_path: PathBuf::from("/bin/bash"),
                    args: vec!["-c", "echo -n B D"].into_iter().map_into().collect(),
                },
                OwnedCommandAndArgs {
                    command_path: PathBuf::from("/bin/bash"),
                    args: vec!["-c", "echo -n B E"].into_iter().map_into().collect(),
                },
            ]
        );
    }
}
