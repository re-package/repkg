use color_eyre::{eyre::eyre, Result};
use semver::Version;

use crate::Project;

/// A PackageProvider, can serve a PACKAGE.repkg file, given a name, or a version
pub trait PackageProvider {
    fn get_latest_project(&self, name: &str) -> Result<&Project>;
    fn get_latest_version(&self, name: &str) -> Result<Version>;
}

pub struct NonePackageProvider;

impl PackageProvider for NonePackageProvider {
    fn get_latest_project(&self, _: &str) -> Result<&Project> {
        todo!()
    }

    fn get_latest_version(&self, _: &str) -> Result<Version> {
        todo!()
    }
}

pub struct MultiPackageProvider {
    pub providers: Vec<Box<dyn PackageProvider>>,
}

impl MultiPackageProvider {
    pub fn init(&mut self, name: &str) -> &mut Self {
        self.providers.sort_by(|x, y| {
            x.get_latest_version(name)
                .unwrap()
                .cmp(&y.get_latest_version(name).unwrap())
        });
        self
    }

    pub fn new(providers: Vec<Box<dyn PackageProvider>>) -> Self {
        Self { providers }
    }
}

impl PackageProvider for MultiPackageProvider {
    fn get_latest_project(&self, name: &str) -> Result<&Project> {
        let latest_provider = self.providers.last().ok_or(eyre!("No provider"))?;

        latest_provider.get_latest_project(name)
    }

    fn get_latest_version(&self, name: &str) -> Result<Version> {
        let latest_provider = self.providers.last().ok_or(eyre!("No provider"))?;

        latest_provider.get_latest_version(name)
    }
}

#[cfg(test)]
mod tests {
    use color_eyre::Result;
    use semver::Version;

    use crate::Project;

    use super::{MultiPackageProvider, PackageProvider};

    pub struct ConstantProvider {
        version: Version,
        project: Project,
    }

    impl ConstantProvider {
        fn new(version: Version, content: Project) -> Self {
            Self {
                version,
                project: content,
            }
        }
    }

    impl PackageProvider for ConstantProvider {
        fn get_latest_project(&self, _name: &str) -> Result<&Project> {
            Ok(&self.project)
        }

        fn get_latest_version(&self, _name: &str) -> Result<Version> {
            Ok(self.version.clone())
        }
    }

    #[test]
    fn multi_package_provider() -> Result<()> {
        let provider1 = ConstantProvider::new(lenient_semver::parse("1.0.0")?, Project::default());
        let provider2 = ConstantProvider::new(lenient_semver::parse("1.1.0")?, Project::default());

        let mut multi_provider =
            MultiPackageProvider::new(vec![Box::new(provider1), Box::new(provider2)]);

        multi_provider.init("anything");

        assert_eq!(
            multi_provider.get_latest_version("anything")?,
            lenient_semver::parse("1.1.0")?
        );

        Ok(())
    }
}
