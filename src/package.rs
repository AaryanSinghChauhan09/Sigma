//! Package manager dependency resolver for SigmaOS component ecosystem.
//! Inspired by SigmaOS pacman and AUR dependency resolution.
//! Applies fail-fast validation and clear abstraction per SigmaOS principles.

use std::collections::{BTreeMap, HashSet};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageError {
    CircularDependency(String),
    MissingDependency(String),
    VersionConflict(String),
    InvalidVersion(String),
}

impl fmt::Display for PackageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CircularDependency(pkg) => write!(f, "circular dependency detected: {}", pkg),
            Self::MissingDependency(dep) => write!(f, "missing dependency: {}", dep),
            Self::VersionConflict(msg) => write!(f, "version conflict: {}", msg),
            Self::InvalidVersion(msg) => write!(f, "invalid version: {}", msg),
        }
    }
}

impl std::error::Error for PackageError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl PackageVersion {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self { major, minor, patch }
    }

    pub fn parse(s: &str) -> Result<Self, PackageError> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 3 {
            return Err(PackageError::InvalidVersion(format!("expected 3 parts, got {}", parts.len())));
        }
        Ok(Self {
            major: parts[0].parse().map_err(|_| PackageError::InvalidVersion(s.to_string()))?,
            minor: parts[1].parse().map_err(|_| PackageError::InvalidVersion(s.to_string()))?,
            patch: parts[2].parse().map_err(|_| PackageError::InvalidVersion(s.to_string()))?,
        })
    }

    pub fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }

    pub fn satisfies(&self, constraint: &str) -> bool {
        if constraint == "*" || constraint.is_empty() {
            return true;
        }
        let parts: Vec<&str> = constraint.split('.').collect();
        if parts.len() < 2 {
            return true;
        }
        let major = parts[0].parse().unwrap_or(0);
        let minor = parts[1].parse().unwrap_or(0);
        self.major >= major && self.minor >= minor
    }
}

impl std::cmp::Ord for PackageVersion {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.major, self.minor, self.patch).cmp(&(other.major, other.minor, other.patch))
    }
}

impl std::cmp::PartialOrd for PackageVersion {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: PackageVersion,
    pub dependencies: Vec<(String, String)>, // (name, version_constraint)
    pub provides: Vec<String>,
    pub conflicts: Vec<String>,
}

impl Package {
    pub fn new(name: impl Into<String>, version: PackageVersion) -> Self {
        Self {
            name: name.into(),
            version,
            dependencies: vec![],
            provides: vec![],
            conflicts: vec![],
        }
    }

    pub fn with_dependency(mut self, name: impl Into<String>, constraint: impl Into<String>) -> Self {
        self.dependencies.push((name.into(), constraint.into()));
        self
    }

    pub fn with_provides(mut self, name: impl Into<String>) -> Self {
        self.provides.push(name.into());
        self
    }

    pub fn with_conflicts(mut self, name: impl Into<String>) -> Self {
        self.conflicts.push(name.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct PackageManager {
    pub packages: BTreeMap<String, Package>,
    pub installed: BTreeMap<String, PackageVersion>,
    pub resolver_cache: BTreeMap<String, Vec<String>>,
}

impl Default for PackageManager {
    fn default() -> Self {
        Self {
            packages: BTreeMap::new(),
            installed: BTreeMap::new(),
            resolver_cache: BTreeMap::new(),
        }
    }
}

impl PackageManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_package(&mut self, package: Package) {
        self.packages.insert(package.name.clone(), package);
    }

    pub fn install(&mut self, name: &str) -> Result<Vec<String>, PackageError> {
        if self.installed.contains_key(name) {
            return Ok(vec![]);
        }
        
        let package = self.packages.get(name)
            .ok_or_else(|| PackageError::MissingDependency(name.to_string()))?
            .clone();
        
        let mut install_order = vec![];
        let mut visited = HashSet::new();
        let mut visiting = HashSet::new();
        
        self.resolve_dependencies(name, &mut visited, &mut visiting, &mut install_order)?;
        
        for pkg_name in &install_order {
            if let Some(pkg) = self.packages.get(pkg_name) {
                self.installed.insert(pkg.name.clone(), pkg.version.clone());
            }
        }
        
        Ok(install_order)
    }

    fn resolve_dependencies(
        &self,
        name: &str,
        visited: &mut HashSet<String>,
        visiting: &mut HashSet<String>,
        order: &mut Vec<String>,
    ) -> Result<(), PackageError> {
        if visited.contains(name) {
            return Ok(());
        }
        
        if visiting.contains(name) {
            return Err(PackageError::CircularDependency(name.to_string()));
        }
        
        visiting.insert(name.to_string());
        
        let package = self.packages.get(name)
            .ok_or_else(|| PackageError::MissingDependency(name.to_string()))?;
        
        for (dep_name, dep_constraint) in &package.dependencies {
            if let Some(dep_package) = self.packages.get(dep_name) {
                if !dep_package.version.satisfies(dep_constraint) {
                    return Err(PackageError::VersionConflict(format!(
                        "{} requires {} {} but {} is installed",
                        name, dep_name, dep_constraint, dep_package.version.to_string()
                    )));
                }
            }
            self.resolve_dependencies(dep_name, visited, visiting, order)?;
        }
        
        visiting.remove(name);
        visited.insert(name.to_string());
        order.push(name.to_string());
        
        Ok(())
    }

    pub fn uninstall(&mut self, name: &str) -> Result<Vec<String>, PackageError> {
        if !self.installed.contains_key(name) {
            return Ok(vec![]);
        }
        
        let mut remove_order = vec![];
        let mut visited = HashSet::new();
        
        self.find_dependents(name, &mut visited, &mut remove_order)?;
        
        for pkg_name in remove_order.iter().rev() {
            self.installed.remove(pkg_name);
        }
        
        Ok(remove_order)
    }

    fn find_dependents(&self, name: &str, visited: &mut HashSet<String>, order: &mut Vec<String>) -> Result<(), PackageError> {
        if visited.contains(name) {
            return Ok(());
        }
        
        visited.insert(name.to_string());
        order.push(name.to_string());
        
        for (pkg_name, pkg) in &self.packages {
            if pkg.dependencies.iter().any(|(dep, _)| dep == name) {
                self.find_dependents(pkg_name, visited, order)?;
            }
        }
        
        Ok(())
    }

    pub fn check_conflicts(&self, name: &str) -> Vec<String> {
        let package = match self.packages.get(name) {
            Some(p) => p,
            None => return vec![],
        };
        
        let mut conflicts = vec![];
        
        for conflict in &package.conflicts {
            if self.installed.contains_key(conflict) {
                conflicts.push(conflict.clone());
            }
        }
        
        for (installed_name, _) in &self.installed {
            if let Some(installed_pkg) = self.packages.get(installed_name) {
                if installed_pkg.conflicts.contains(&package.name) {
                    conflicts.push(installed_name.clone());
                }
            }
        }
        
        conflicts
    }

    pub fn get_dependency_tree(&self, name: &str) -> BTreeMap<String, Vec<String>> {
        let mut tree = BTreeMap::new();
        
        if let Some(package) = self.packages.get(name) {
            tree.insert(name.to_string(), package.dependencies.iter().map(|(n, _)| n.clone()).collect());
            
            for (dep_name, _) in &package.dependencies {
                if let Some(dep_package) = self.packages.get(dep_name) {
                    tree.insert(dep_name.clone(), dep_package.dependencies.iter().map(|(n, _)| n.clone()).collect());
                }
            }
        }
        
        tree
    }

    pub fn generate_makepkg_config(&self, name: &str) -> Option<String> {
        let package = self.packages.get(name)?;
        
        let mut config = String::new();
        config.push_str(&format!("pkgname={}", package.name));
        config.push_str(&format!("pkgver={}", package.version.to_string()));
        config.push_str(&format!("pkgrel=1"));
        
        if !package.dependencies.is_empty() {
            let deps: Vec<String> = package.dependencies.iter()
                .map(|(n, c)| format!("'{}>={}'", n, c))
                .collect();
            config.push_str(&format!("depends=({})", deps.join(" ")));
        }
        
        Some(config)
    }

    pub fn list_installed(&self) -> Vec<(&String, &PackageVersion)> {
        self.installed.iter().collect()
    }

    pub fn is_installed(&self, name: &str) -> bool {
        self.installed.contains_key(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_version_parse() {
        let v = PackageVersion::parse("1.2.3").unwrap();
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 3);
    }

    #[test]
    fn test_package_version_parse_invalid() {
        assert!(PackageVersion::parse("1.2").is_err());
        assert!(PackageVersion::parse("abc").is_err());
    }

    #[test]
    fn test_package_version_to_string() {
        let v = PackageVersion::new(1, 2, 3);
        assert_eq!(v.to_string(), "1.2.3");
    }

    #[test]
    fn test_package_version_satisfies() {
        let v = PackageVersion::new(1, 2, 3);
        assert!(v.satisfies("1.0"));
        assert!(v.satisfies("1.2"));
        assert!(!v.satisfies("2.0"));
        assert!(v.satisfies("*"));
    }

    #[test]
    fn test_package_version_ord() {
        let v1 = PackageVersion::new(1, 2, 3);
        let v2 = PackageVersion::new(1, 2, 4);
        assert!(v1 < v2);
        assert!(v2 > v1);
    }

    #[test]
    fn test_package_new() {
        let pkg = Package::new("test", PackageVersion::new(1, 0, 0));
        assert_eq!(pkg.name, "test");
        assert_eq!(pkg.version.to_string(), "1.0.0");
    }

    #[test]
    fn test_package_with_dependency() {
        let pkg = Package::new("test", PackageVersion::new(1, 0, 0))
            .with_dependency("dep", "1.0");
        assert_eq!(pkg.dependencies.len(), 1);
        assert_eq!(pkg.dependencies[0].0, "dep");
    }

    #[test]
    fn test_package_manager_new() {
        let mgr = PackageManager::new();
        assert_eq!(mgr.packages.len(), 0);
    }

    #[test]
    fn test_package_manager_add_package() {
        let mut mgr = PackageManager::new();
        let pkg = Package::new("test", PackageVersion::new(1, 0, 0));
        mgr.add_package(pkg);
        assert_eq!(mgr.packages.len(), 1);
    }

    #[test]
    fn test_package_manager_install() {
        let mut mgr = PackageManager::new();
        mgr.add_package(Package::new("base", PackageVersion::new(1, 0, 0)));
        mgr.add_package(Package::new("app", PackageVersion::new(1, 0, 0))
            .with_dependency("base", "1.0"));
        
        let order = mgr.install("app").unwrap();
        assert_eq!(order.len(), 2);
        assert!(order.contains(&"base".to_string()));
        assert!(order.contains(&"app".to_string()));
    }

    #[test]
    fn test_package_manager_install_missing_dependency() {
        let mut mgr = PackageManager::new();
        mgr.add_package(Package::new("app", PackageVersion::new(1, 0, 0))
            .with_dependency("missing", "1.0"));
        
        let result = mgr.install("app");
        assert!(result.is_err());
    }

    #[test]
    fn test_package_manager_install_circular_dependency() {
        let mut mgr = PackageManager::new();
        mgr.add_package(Package::new("a", PackageVersion::new(1, 0, 0))
            .with_dependency("b", "1.0"));
        mgr.add_package(Package::new("b", PackageVersion::new(1, 0, 0))
            .with_dependency("a", "1.0"));
        
        let result = mgr.install("a");
        assert!(result.is_err());
    }

    #[test]
    fn test_package_manager_uninstall() {
        let mut mgr = PackageManager::new();
        mgr.add_package(Package::new("base", PackageVersion::new(1, 0, 0)));
        mgr.add_package(Package::new("app", PackageVersion::new(1, 0, 0))
            .with_dependency("base", "1.0"));
        
        mgr.install("app").unwrap();
        assert!(mgr.is_installed("app"));
        let removed = mgr.uninstall("app").unwrap();
        assert!(removed.contains(&"app".to_string()));
        assert!(!mgr.is_installed("app"));
    }

    #[test]
    fn test_package_manager_check_conflicts() {
        let mut mgr = PackageManager::new();
        mgr.add_package(Package::new("pkg1", PackageVersion::new(1, 0, 0)));
        mgr.add_package(Package::new("pkg2", PackageVersion::new(1, 0, 0))
            .with_conflicts("pkg1"));
        
        mgr.install("pkg1").unwrap();
        let conflicts = mgr.check_conflicts("pkg2");
        assert_eq!(conflicts.len(), 1);
        assert_eq!(conflicts[0], "pkg1");
    }

    #[test]
    fn test_generate_makepkg_config() {
        let mut mgr = PackageManager::new();
        mgr.add_package(Package::new("test", PackageVersion::new(1, 0, 0))
            .with_dependency("dep", "1.0"));
        
        let config = mgr.generate_makepkg_config("test").unwrap();
        assert!(config.contains("pkgname=test"));
        assert!(config.contains("depends="));
    }

    #[test]
    fn test_is_installed() {
        let mut mgr = PackageManager::new();
        mgr.add_package(Package::new("test", PackageVersion::new(1, 0, 0)));
        
        assert!(!mgr.is_installed("test"));
        mgr.install("test").unwrap();
        assert!(mgr.is_installed("test"));
    }
}
