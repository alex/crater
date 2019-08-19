use crate::cmd::SandboxImage;
use failure::{Error, ResultExt};
use std::path::{Path, PathBuf};
use std::time::Duration;

#[cfg(windows)]
static DEFAULT_SANDBOX_IMAGE: &str = "rustops/crates-build-env-windows";

#[cfg(not(windows))]
static DEFAULT_SANDBOX_IMAGE: &str = "rustops/crates-build-env";

const DEFAULT_COMMAND_TIMEOUT: Option<Duration> = Some(Duration::from_secs(15 * 60));
const DEFAULT_COMMAND_NO_OUTPUT_TIMEOUT: Option<Duration> = None;

/// Builder of a [`Workspace`](struct.Workspace.html).
pub struct WorkspaceBuilder {
    path: PathBuf,
    sandbox_image: Option<SandboxImage>,
    command_timeout: Option<Duration>,
    command_no_output_timeout: Option<Duration>,
}

impl WorkspaceBuilder {
    /// Create a new builder.
    ///
    /// The provided path will be the home of the workspace, containing all the data generated by
    /// rustwide (including state and caches).
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.into(),
            sandbox_image: None,
            command_timeout: DEFAULT_COMMAND_TIMEOUT,
            command_no_output_timeout: DEFAULT_COMMAND_NO_OUTPUT_TIMEOUT,
        }
    }

    /// Override the image used for sandboxes.
    ///
    /// By default rustwide will use the [rustops/crates-build-env] image on Linux systems, and
    /// [rustops/crates-build-env-windows] on Windows systems. Those images contain dependencies to
    /// build a large amount of crates.
    ///
    /// [rustops/crates-build-env]: https://hub.docker.com/rustops/crates-build-env
    /// [rustops/crates-build-env-windows]: https://hub.docker.com/rustops/crates-build-env-windows
    pub fn sandbox_image(mut self, image: SandboxImage) -> Self {
        self.sandbox_image = Some(image);
        self
    }

    /// Set the default timeout of [`Command`](cmd/struct.Command.html), which can be overridden
    /// with the [`Command::timeout`](cmd/struct.Command.html#method.timeout) method. To disable
    /// the timeout set its value to `None`. By default the timeout is 15 minutes.
    pub fn command_timeout(mut self, timeout: Option<Duration>) -> Self {
        self.command_timeout = timeout;
        self
    }

    /// Set the default no output timeout of [`Command`](cmd/struct.Command.html), which can be
    /// overridden with the
    /// [`Command::no_output_timeout`](cmd/struct.Command.html#method.no_output_timeout) method. To
    /// disable the timeout set its value to `None`. By default it's disabled.
    pub fn command_no_output_timeout(mut self, timeout: Option<Duration>) -> Self {
        self.command_no_output_timeout = timeout;
        self
    }

    /// Initialize the workspace. This will create all the necessary local files and fetch the rest from the network. It's
    /// not unexpected for this method to take minutes to run on slower network connections.
    pub fn init(self) -> Result<Workspace, Error> {
        std::fs::create_dir_all(&self.path).with_context(|_| {
            format!(
                "failed to create workspace directory: {}",
                self.path.display()
            )
        })?;

        let sandbox_image = if let Some(img) = self.sandbox_image {
            img
        } else {
            SandboxImage::remote(DEFAULT_SANDBOX_IMAGE)?
        };

        Ok(Workspace {
            path: self.path,
            sandbox_image,
            command_timeout: self.command_timeout,
            command_no_output_timeout: self.command_no_output_timeout,
        })
    }
}

/// Directory on the filesystem containing rustwide's state and caches.
///
/// Use [`WorkspaceBuilder`](struct.WorkspaceBuilder.html) to create a new instance of it.
pub struct Workspace {
    path: PathBuf,
    sandbox_image: SandboxImage,
    command_timeout: Option<Duration>,
    command_no_output_timeout: Option<Duration>,
}

impl Workspace {
    pub(crate) fn cargo_home(&self) -> PathBuf {
        self.path.join("local").join("cargo-home")
    }

    pub(crate) fn rustup_home(&self) -> PathBuf {
        self.path.join("local").join("rustup-home")
    }

    pub(crate) fn sandbox_image(&self) -> &SandboxImage {
        &self.sandbox_image
    }

    pub(crate) fn default_command_timeout(&self) -> Option<Duration> {
        self.command_timeout
    }

    pub(crate) fn default_command_no_output_timeout(&self) -> Option<Duration> {
        self.command_no_output_timeout
    }
}
