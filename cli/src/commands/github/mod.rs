mod pr;

use self::pr::cmd_github_pr;
use crate::cli_util::CommandHelper;
use crate::command_error::CommandError;
use crate::commands::github::pr::PRCommand;
use crate::ui::Ui;
use clap::Subcommand;

#[derive(Subcommand, Clone, Debug)]
pub enum GitHubCommand {
    #[command(subcommand)]
    PR(PRCommand),
}

pub fn cmd_github(
    ui: &mut Ui,
    command: &CommandHelper,
    subcommand: &GitHubCommand,
) -> Result<(), CommandError> {
    match subcommand {
        GitHubCommand::PR(args) => cmd_github_pr(ui, command, args),
    }
}
