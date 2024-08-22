use std::fmt::Display;

pub enum Language {
    Rust,
    Bicep,
    Typescript,
    Javascript,
    Lua,
    C,
    Cpp,
    CSharp,
    Go,
    Java,
    Kotlin,
    Header,
    Swift,
    Python,
    Ruby,
    Perl,
    Php,
    Erlang,
    Elixir,
    Haskell,
    Clojure,
    Svelte,
    Html,
    Css,
    Markdown,
    Yaml,
    Json,
    Toml,
    Zig,
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::Rust => write!(f, "rust"),
            Language::Bicep => write!(f, "bicep"),
            Language::Typescript => write!(f, "ts"),
            Language::Javascript => write!(f, "js"),
            Language::Lua => write!(f, "lua"),
            Language::C => write!(f, "c"),
            Language::Cpp => write!(f, "cpp"),
            Language::CSharp => write!(f, "cs"),
            Language::Go => write!(f, "go"),
            Language::Java => write!(f, "java"),
            Language::Kotlin => write!(f, "kt"),
            Language::Header => write!(f, "h"),
            Language::Swift => write!(f, "swift"),
            Language::Python => write!(f, "pu"),
            Language::Ruby => write!(f, "rb"),
            Language::Perl => write!(f, "pl"),
            Language::Php => write!(f, "php"),
            // TODO add hrl
            Language::Erlang => write!(f, "erl"),
            // TODO add exs
            Language::Elixir => write!(f, "ex"),
            Language::Haskell => write!(f, "hs"),
            Language::Clojure => write!(f, "clj"),
            Language::Svelte => write!(f, "svelte"),
            Language::Html => write!(f, "html"),
            Language::Css => write!(f, "css"),
            Language::Markdown => write!(f, "md"),
            Language::Yaml => write!(f, "yml"),
            Language::Json => write!(f, "json"),
            Language::Toml => write!(f, "toml"),
            Language::Zig => write!(f, "zig"),
        }
    }
}

pub struct Lang;

impl Lang {
    pub fn from_str(s: &str) -> Option<Language> {
        match s {
            "rs" => Some(Language::Rust),
            "bicep" => Some(Language::Bicep),
            "ts" => Some(Language::Typescript),
            "js" => Some(Language::Javascript),
            "lua" => Some(Language::Lua),
            "c" => Some(Language::C),
            "cpp" => Some(Language::Cpp),
            "cs" => Some(Language::CSharp),
            "go" => Some(Language::Go),
            "java" => Some(Language::Java),
            "kt" => Some(Language::Kotlin),
            "h" => Some(Language::Header),
            "swift" => Some(Language::Swift),
            "py" => Some(Language::Python),
            "rb" => Some(Language::Ruby),
            "pl" => Some(Language::Perl),
            "php" => Some(Language::Php),
            "erl" => Some(Language::Erlang),
            "ex" => Some(Language::Elixir),
            "hs" => Some(Language::Haskell),
            "clj" => Some(Language::Clojure),
            "svelte" => Some(Language::Svelte),
            "html" => Some(Language::Html),
            "css" => Some(Language::Css),
            "md" => Some(Language::Markdown),
            "yml" => Some(Language::Yaml),
            "json" => Some(Language::Json),
            "toml" => Some(Language::Toml),
            "zig" => Some(Language::Zig),
            _ => None,
        }
    }
}

pub struct LCount {
    lines: usize,
    nb_files: usize,
    bytes: usize,
}

impl LCount {
    pub fn new() -> Self {
        Self {
            lines: 0,
            nb_files: 0,
            bytes: 0,
        }
    }

    pub fn add_line(&mut self) {
        self.lines += 1;
    }

    pub fn add_file(&mut self) {
        self.nb_files += 1;
    }

    pub fn add_bytes(&mut self, bytes: usize) {
        self.bytes += bytes;
    }
}
