use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

use toml_edit::{value, Item, Table};

/// ## Rust Dependence
/// 描述Cargo.toml中的依赖的写法
/// 
/// format: `name = { version = "0.1.0", features = ["feature1", "feature2"], default-features = false, git/path = "git/path", branch = "git branch", rev = "git rev", tag = "git tag"}`
/// ### Example
/// ```rust
/// let mut makepad_widget = RustDependence::new("makepad-widgets");
/// makepad_widget.set_ty(DepType::local(
///     "E:/Rust/try/makepad/makepad/rik/makepad/widgets",
/// ));
/// ```
#[derive(Debug, Clone)]
pub struct RustDependence {
    pub name: String,
    pub version: Option<String>,
    pub features: Option<Vec<String>>,
    pub default_features: Option<bool>,
    pub ty: DepType,
}

impl RustDependence {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            version: None,
            features: None,
            default_features: None,
            ty: DepType::Crate,
        }
    }
    pub fn set_version(&mut self, version: &str) -> &mut Self {
        self.version.replace(version.to_string());
        self
    }
    pub fn set_features(&mut self, features: Vec<String>) -> &mut Self {
        self.features.replace(features);
        self
    }
    pub fn set_default_features(&mut self, default_features: bool) -> &mut Self {
        self.default_features.replace(default_features);
        self
    }
    pub fn set_ty(&mut self, ty: DepType) -> &mut Self {
        self.ty = ty;
        self
    }
    /// convert to toml edit table value
    /// which can call insert fn when need to insert into `Table`
    /// return (name, value)
    pub fn to_table_value(&self) -> (String, Item) {
        let mut item = Item::Table(Table::new());

        match &self.ty {
            DepType::Crate => (),
            DepType::Remote(remote) => {
                item["git"] = value(remote.url.as_str());
                if let Some(branch) = remote.branch.as_ref() {
                    item["branch"] = value(branch);
                }
                if let Some(rev) = remote.rev.as_ref() {
                    item["rev"] = value(rev);
                }
                if let Some(tag) = remote.tag.as_ref() {
                    item["tag"] = value(tag);
                }
            }
            DepType::Local(local) => {
                item["path"] = value(local.to_str().unwrap());
            }
        }

        if let Some(version) = self.version.as_ref() {
            item["version"] = value(version);
        }
        if let Some(features) = self.features.as_ref() {
            item["features"] = value(features.join(", "));
        }
        if let Some(default_features) = self.default_features.as_ref() {
            item["default-features"] = value(*default_features);
        }

        (self.name.to_string(), item)
    }
}

/// ## The type of dependence
/// 
/// - Crate
/// - Remote
/// - Local
#[derive(Debug, Clone)]
pub enum DepType {
    /// crate 表示来自crates.io的依赖使用cargo install安装
    Crate,
    /// remote 表示来自远程的依赖, 可能是Github等
    Remote(RemoteDep),
    /// local 表示本地的依赖
    Local(PathBuf),
}

impl DepType {
    pub fn local<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        DepType::Local(path.as_ref().to_path_buf())
    }
}

impl Display for DepType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DepType::Crate => write!(f, ""),
            DepType::Remote(remote) => remote.fmt(f),
            DepType::Local(local) => {
                f.write_fmt(format_args!("path = \"{}\"", local.to_str().unwrap()))
            }
        }
    }
}

/// ## Git remote dependence
/// format: `url = "git/url", branch = "git branch", rev = "git rev", tag = "git tag"`
/// ### Example
/// ```toml
/// serde = { git = "https://serde/git/url", branch = "master" }
/// ```
#[derive(Debug, Clone)]
pub struct RemoteDep {
    pub url: String,
    pub branch: Option<String>,
    /// HEAD commit of PR (SHA1 hash)
    pub rev: Option<String>,
    pub tag: Option<String>,
}

impl RemoteDep {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            branch: None,
            rev: None,
            tag: None,
        }
    }
}

impl Display for RemoteDep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut other = String::new();

        if let Some(branch) = self.branch.as_ref() {
            other.push_str(&format!("branch = \"{}\", ", branch));
        }
        if let Some(rev) = self.rev.as_ref() {
            other.push_str(&format!("rev = \"{}\", ", rev));
        }
        if let Some(tag) = self.tag.as_ref() {
            other.push_str(&format!("tag = \"{}\", ", tag));
        }

        f.write_fmt(format_args!("git = \"{}\"", self.url))
    }
}

#[cfg(test)]
mod test_dep {
    use toml_edit::{value, Item, Table};

    #[test]
    fn toml_item() {
        let mut item = Item::Table(Table::new());
        item["version"] = value("0.1.0");
        // item.as_inline_table_mut().map(|t| t.fmt());
        dbg!(item.to_string());
    }
}
