# Setting up VSCode for rust programming

## Installing the rust tool chain

```bash
curl https://sh.rustup.rs -sSf | sh
# By default the tools are installed to the user `HOME` directory.
# Add the below to .bashrc or your shell config file
# reload the shell
source "$HOME/.cargo/env"

# verify installation
cargo --version
rustup --version
rustc -V
```

## Uninstall

`rustup self uninstall` removes the entire installed rust toolchain.

## Setup VScode for Rust programming

Extensions required/recommended

- [rust-analyzer](https://rust-analyzer.github.io/)
- Rust Test explorer
- Rust Doc Viewer
- Rust Extension Pack (Includes **crates** and **BetterTOML** extension inaddition to rust-analyzer)
- CodeLLDB for debugging

### Workspace configuration

- In the project/git repository root folder, create a `.code-workspace` file with the following contents.

```JSON
{
  "folders": [
    {
      "path": "."
    }
  ],
  "settings": {
    "editor.renderWhitespace": "all",

    "[markdown]": {
      "editor.codeActionsOnSave": {
        "source.fixAll.markdownlint": true
      },
      "editor.wordWrap": "wordWrapColumn",
      "editor.wrappingIndent": "none"
    },
    "[python]": {
      "editor.wordWrapColumn": 100
    },
    "editor.tabSize": 4,
    "editor.insertSpaces": true,
    "editor.wordWrap": "wordWrapColumn",
    "editor.wordWrapColumn": 100,
    "editor.detectIndentation": false,
    "editor.fontSize": 18,
    "editor.fontFamily": "Monaco monospace",

    "prettier.configPath": ".prettierrc.json",
    "terminal.explorerKind": "integrated",
    "terminal.integrated.defaultProfile.linux": "bash",
    "terminal.integrated.profiles.linux": {
      "bash": {
        "path": "bash",
        "args": ["-l"]
      }
    },
    "terminal.integrated.defaultProfile.osx": "bash",
    "terminal.integrated.profiles.osx": {
      "bash": {
        "path": "bash",
        "args": ["-l"]
      }
    },
    "terminal.integrated.fontSize": 16,
    "terminal.integrated.fontFamily": "monospace",
    "terminal.integrated.env.osx": {
      "RUSTUP_INSTALLATION_DIRECTORY": "${env:HOME}/.rustup",
      "TOOLCHAIN": "nightly-aarch64-apple-darwin"
    },
    "terminal.integrated.env.linux": {
      "RUSTUP_INSTALLATION_DIRECTORY": "${env:HOME}/.rustup",
      "TOOLCHAIN": "nightly-x86_64-unknown-linux-gnu" // use `rustup toolchain list`
    },
    "git.autorefresh": true,
    "markdownlint.ignore": ["**/*.md.html"],
    "files.insertFinalNewline": true,
    "files.associations": {
      "*.md.html": "markdown"
    },
    "shellcheck.enable": true,
    "shellcheck.useWorkspaceRootAsCwd": true,
    "shellcheck.run": "onSave",
    "shellformat.flag": "-i 2 -ci -sr -bn",
    "rust-analyzer.linkedProjects": [
      "${workspaceFolder}/tryouts/rust_handbook_chapters/Cargo.toml"
    ],
    "rust-analyzer.debug.engine": "vadimcn.vscode-lldb",
    "rust-analyzer.debug.openDebugPane": true,
    "rust-analyzer.rustfmt.rangeFormatting.enable": true,
    "rust-analyzer.cargo.autoreload": true,
    // "rust-analyzer.checkOnSave.overrideCommand": [
    //   "cargo",
    //   "clippy",
    //   "--fix",
    //   "--workspace",
    //   "--message-format=json",
    //   "--all-targets",
    //   "--allow-dirty",
    //   "--allow-no-vcs"
    // ],
    // "rust-analyzer.rustfmt.overrideCommand": null,
    "editor.semanticTokenColorCustomizations": {
      "rules": {
        "*.mutable": {
          // set to empty string to disable underline, which is the default
          "fontStyle": "underline strikethrough"
        }
      }
    }
  },
  // https://gist.github.com/deadalusai/9e13e36d61ec7fb72148
  "tasks": {
    "version": "2.0.0",
    "tasks": [
      {
        "label": "Rust: Debug build",
        "command": "cargo",
        "type": "shell",
        "args": ["build", "--bin", "rust_learning"],
        "options": {
          "cwd": "${workspaceFolder}/tryouts/rust_handbook_chapters"
        },
        "problemMatcher": ["$rustc"],
        "presentation": {
          "reveal": "always"
        },
        "group": "build"
      }
    ]
  },
  "launch": {
    "version": "0.2.0",
    "configurations": [
      {
        "type": "lldb",
        "request": "launch",
        "name": "Debug rust file",
        "preLaunchTask": "Rust: Debug build",
        "program": "${workspaceFolder}/tryouts/rust_handbook_chapters/target/debug/rust_learning",
        "args": [],
        "cwd": "${workspaceFolder}/tryouts/rust_handbook_chapters/target/debug",
        "sourceLanguages": ["rust"],
        "env": {
          "RUST_BACKTRACE": "short",
          "APPLICATION_INSIGHTS_NO_DIAGNOSTIC_CHANNEL": "true"
        },
        "terminal": "integrated"
      }
    ]
  }
}
```

## Useful Crates for rust development

- rustfmt - Source code formatter
- clippy - linter
- cargo - package manager and build tool
- rust-docs - Code documentation.

## References

- [Rust installation in linux](https://www.digitalocean.com/community/tutorials/install-rust-on-ubuntu-linux)
- [Setup rust programming in vscode](https://code.visualstudio.com/docs/languages/rust)
