#[cfg(not(target_family = "wasm"))]
pub use argmax::Command;

#[cfg(target_family = "wasm")]
#[derive(Debug)]
pub struct Command(std::process::Command);

#[cfg(target_family = "wasm")]
impl Command {
    pub fn new<S: AsRef<std::ffi::OsStr>>(program: S) -> Self {
        Self(std::process::Command::new(program))
    }

    pub fn stdin(&mut self, cfg: std::process::Stdio) -> &mut Self {
        self.0.stdin(cfg);
        self
    }

    pub fn stdout(&mut self, cfg: std::process::Stdio) -> &mut Self {
        self.0.stdout(cfg);
        self
    }

    pub fn stderr(&mut self, cfg: std::process::Stdio) -> &mut Self {
        self.0.stderr(cfg);
        self
    }

    pub fn try_arg<S: AsRef<std::ffi::OsStr>>(&mut self, arg: S) -> std::io::Result<&mut Self> {
        self.0.arg(arg);
        Ok(self)
    }

    pub fn try_args<I, S>(&mut self, args: I) -> std::io::Result<&mut Self>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<std::ffi::OsStr>,
    {
        self.0.args(args);
        Ok(self)
    }

    pub fn args_would_fit<I, S>(&self, _args: I) -> bool
    where
        I: IntoIterator<Item = S>,
        S: AsRef<std::ffi::OsStr>,
    {
        true
    }

    pub fn status(&mut self) -> std::io::Result<std::process::ExitStatus> {
        self.0.status()
    }

    pub fn output(&mut self) -> std::io::Result<std::process::Output> {
        self.0.output()
    }

    pub fn spawn(&mut self) -> std::io::Result<std::process::Child> {
        self.0.spawn()
    }

    pub fn get_program(&self) -> &std::ffi::OsStr {
        self.0.get_program()
    }
}
