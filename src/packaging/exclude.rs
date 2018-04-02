use std::path::Path;
use std::vec::Vec;

use globset;
use pathdiff::diff_paths;

use error;

pub struct ExcludedFiles {
    glob: globset::GlobSet,
}

impl ExcludedFiles {
    pub fn new(excludes: &Vec<String>) -> Result<Self, error::Error> {
        let mut builder = globset::GlobSetBuilder::new();
        for excl in excludes {
            let glob =
                globset::GlobBuilder::new(excl)
                    .literal_separator(true)
                    .build()
                    .map_err(|_| {
                                 error::Error::Other(format!("could not build exclude for {}",
                                                             excl))
                             })?;
            builder.add(glob);
        }
        let set =
            builder
                .build()
                .map_err(|e| error::Error::Other(format!("could not build glob set: {}", e)))?;

        Ok(ExcludedFiles { glob: set })
    }

    pub fn is_excluded(&self, path: &Path, base: &Path) -> bool {
        diff_paths(path, base)
            .map(|normalized| !self.glob.matches(&normalized).is_empty())
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_glob() {
        let rules = vec![".git".to_string()];

        let excludes = ExcludedFiles::new(&rules);

        assert!(excludes.is_ok());
    }

    #[test]
    fn test_path_separator() {
        let rules = vec!["js/*.js".to_string()];

        let excludes = ExcludedFiles::new(&rules).unwrap();

        assert!(!excludes.is_excluded(&Path::new("build/artefacts/app/js/build/build.js"),
                                      &Path::new("build/artefacts/app")));
        assert!(excludes.is_excluded(&Path::new("build/artefacts/app/js/init.js"),
                                     &Path::new("build/artefacts/app")));
    }

}
