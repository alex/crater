use errors::*;

pub trait Process<S> {
    fn process(self, s: S) -> Result<(S, Vec<Self>)> where Self: Sized;
}

pub trait Arguable: Sized {
    fn from_args(args: Vec<String>) -> Result<Self>;
    fn to_args(self) -> Vec<String>;
}

pub fn run<S, C>(mut state: S, cmd: C) -> Result<S>
    where C: Process<S>, C: Arguable
{
    let mut cmds = vec!(cmd);
    loop {
        if let Some(cmd) = cmds.pop() {

            // Round trip through command line argument parsing,
            // just for testing purpose.
            let cmd: Vec<String> = cmd.to_args();
            let cmd: C = Arguable::from_args(cmd)?;

            let (state_, new_cmds) = cmd.process(state)?;
            state = state_;

            // Each command execution returns a list of new commands
            // to execute, in order, before considering the original
            // complete.
            cmds.extend(new_cmds.into_iter().rev());
        } else {
            break;
        }
    }

    Ok(state)
}