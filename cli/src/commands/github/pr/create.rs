use crate::cli_util::CommandHelper;
use crate::cli_util::RevisionArg;
use crate::command_error::CommandError;
use crate::complete;
use crate::ui::Ui;
use clap_complete::ArgValueCompleter;

#[derive(clap::Args, Clone, Debug)]
pub struct GitHubPRCreateArgs {
    /// Push bookmarks pointing to these commits (can be repeated)
    #[arg(
        long,
        short,
        value_name = "REVSET",
        // While `-r` will often be used with mutable revisions, immutable
        // revisions can be useful as parts of revsets or to push
        // special-purpose branches.
        add = ArgValueCompleter::new(complete::revset_expression_all),
    )]
    revision: RevisionArg,
}

pub fn cmd_github_pr_create(
    ui: &mut Ui,
    command: &CommandHelper,
    args: &GitHubPRCreateArgs,
) -> Result<(), CommandError> {
    Ok(())
}
