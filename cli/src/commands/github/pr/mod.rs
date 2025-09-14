mod create;

use self::create::{GitHubPRCreateArgs, cmd_github_pr_create};
use crate::cli_util::CommandHelper;
use crate::command_error::CommandError;
use crate::ui::Ui;
use clap::Subcommand;

#[derive(Subcommand, Clone, Debug)]
pub enum PRCommand {
    Create(GitHubPRCreateArgs),
}

pub fn cmd_github_pr(
    ui: &mut Ui,
    command: &CommandHelper,
    subcommand: &PRCommand,
) -> Result<(), CommandError> {
    match subcommand {
        PRCommand::Create(args) => cmd_github_pr_create(ui, command, args),
    }
}
