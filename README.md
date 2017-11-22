# Krankerl

[![Build Status](https://travis-ci.org/ChristophWurst/krankerl.svg?branch=master)](https://travis-ci.org/ChristophWurst/krankerl)

A CLI helper to manage [Nextcloud](https://nextcloud.com/) apps.

```
Usage:
  krankerl enable
  krankerl disable
  krankerl init
  krankerl list apps <version>
  krankerl list categories
  krankerl login [--appstore | --github] <token>
  krankerl package
  krankerl publish (--nightly) <url>
  krankerl sign --package
  krankerl --version

Options:
  -h --help     Show this screen.
  --version     Show version.
```

## Enable the current app
Krankerl provides a shortcut to enabling an app via the `occ` tool. This assumes
that you are inside the app's root directory and `occ` can be found in the directory
two levels above the current one.

```bash
krankerl enable
```

## Disable the current app
Krankerl provides a shortcut to enabling an app via the `occ` tool. This assumes
that you are inside the app's root directory and `occ` can be found in the directory
two levels above the current one.

```bash
krankerl disable
```

## Packaging
Krankerl can build a `.tar.gz` archive for the current app. This requires a
`krankerl.toml` configuration file to exist in the app's root directory.
Moreover, Krankerl will not use the current state of the app directory, but
clone it into a new directory. This step was added to make app builds
reproducible and independent of local changes.

## Steps

These are the steps Krankerl executes to package an app:

* Delete `build/artifacts` if it exists
* Clone current directory to `build/artifacts/<app_id>`
* Run pre-packaging commands
* Build list of files and directories that are not excluded by any `exclude` rule
* Pack and compress those files and directories into a `build/artifacts/<app_id>.tar.gz` archive

### Initialize configuration
You can either manually create the `krankerl.toml` config file or have Krankerl
create it for you by using the `init` command:

```bash
krankerl init
```

This will create a minimal configuration. Adjust it to your needs.

### Configuration overview
#### Excluded files

Certain files and directories of your repository shouldn't be part of the
generated tarball. The `exclude` array in the `[packaging]` lists `glob`
patterns of files and directories to exclude.

Typical excludes are the `.git` directory, tests and configuration files that are
only required during development.

```toml
[package]
exclude = [
    ".git",
    "composer.json",
    "composer.lock",
    "krankerl.toml",
    "node_modules",
    "tests",
]
```
#### Pre-package commands

Building app archives often requires execution of a few commands. Common
examples for that are composer and npm dependencies that are not part of
the git repository and thus are missing in the cloned directory.

The `package_cmds` array will let you specify commands that are executed
by `sh` in the app's root directory.

```toml
[package]
before_cmds = [
    "composer install",
    "npm install",
    "npm run build",
]
```
