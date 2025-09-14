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
        default_value = "@",
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
    let workspace_command = command.workspace_helper(ui)?;
    let rev = workspace_command.resolve_single_rev(ui, &args.revision)?;

    // assert gh installed and auth'd for repo (common method for future jj github pr commands)

    // assert yes and dry_run not true at the same time

    // rev_bookmark = bookmark if rev has bookmark else None
    // bookmark_pushed = if rev_has_bookmark -> check if bookmark is pushed else False
    // If not bookmark_pushed:
    //   Push remote = Use `--remote` flag, `git.push` setting, or fall back to `origin`
    // else:
    //   Push remote = already pushed remote
    //   If pushed to multiple places -> error
    // If base specified:
    //   base branch = base
    //   base remote = base remote if specified else push remote
    // else:
    //   base branch = look for ancestor with pushed bookmark else trunk
    //   base remote = pushed bookmark remote if not pushed to multiple places else error
    // pr_title = commit short desc
    // pr_description = pr template file if it exists or if not, full commit description
    // print preview
    // if dry run:
    //   Ok(())
    // if not yes:
    //   ask user Y/N to confirm
    //   if N, Ok(())
    // if not rev_bookmark and not bookmark_pushed:
    //   same logic as `jj git push -c <rev>`
    //   rev_bookmark = rev bookmark
    // elif not bookmark_pushed
    //   same logic as `jj git push -b <bookmark>`
    // gh pr create --repo=<base remote> --base=<base branch> --head=<push remote / rev bookmark> --title=<pr title> --body=<pr description>
    Ok(())
}
