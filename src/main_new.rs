use std::collections::HashMap;
use url::Url;

type UrlCheck = dyn Fn(&Url) -> bool;

fn main() {
    let checks: Vec<Box<UrlCheck>> = vec![
        Box::new(|u: &Url| {
            let mut interesting = 0;
            for (k, vv) in u.query_pairs() {
                for (_, v) in vv {
                    if qs_check(&k, &v) {
                        interesting += 1;
                    }
                }
            }
            interesting > 0
        }),
        Box::new(|u: &Url| {
            let exts = [
                ".php", ".phtml", ".asp", ".aspx", ".asmx", ".ashx", ".cgi", ".pl", ".json",
                ".xml", ".rb", ".py", ".sh", ".yaml", ".yml", ".toml", ".ini", ".md", ".mkd",
                ".do", ".jsp", ".jspa",
            ];
            let p = u.escaped_path().to_lowercase();
            for e in &exts {
                if p.ends_with(e) {
                    return true;
                }
            }
            false
        }),
        Box::new(|u: &Url| {
            let p = u.escaped_path().to_lowercase();
            p.contains("ajax")
                || p.contains("jsonp")
                || p.contains("admin")
                || p.contains("include")
                || p.contains("src")
                || p.contains("redirect")
                || p.contains("proxy")
                || p.contains("test")
                || p.contains("tmp")
                || p.contains("temp")
        }),
        Box::new(|u: &Url| {
            u.port() != "80" && u.port() != "443" && !u.port().is_empty()
        }),
    ];

    let mut seen: HashMap<String, bool> = HashMap::new();

    let stdin = std::io::stdin();
    let handle = stdin.lock();
    let lines = handle.lines().filter_map(Result::ok);

    for line in lines {
        if let Ok(u) = Url::parse(&line) {
            if is_boring_static_file(&u) {
                continue;
            }

            let mut pp: Vec<_> = u.query_pairs().map(|(k, _)| k.into_owned()).collect();
            pp.sort();

            let key = format!(
                "{}{}?{}",
                u.host_str().unwrap_or(""),
                u.escaped_path(),
                pp.join("&")
            );

            if seen.contains_key(&key) {
                continue;
            }
            seen.insert(key, true);

            let mut interesting = 0;

            for check in &checks {
                if check(&u) {
                    interesting += 1;
                }
            }

            if interesting > 0 {
                println!("{}", line);
            }
        }
    }
}

fn qs_check(k: &str, v: &str) -> bool {
    let k = k.to_lowercase();
    let v = v.to_lowercase();

    if k.starts_with("utm_") {
        return false;
    }

    v.starts_with("http")
        || v.contains('{')
        || v.contains('[')
        || v.contains('/')
        || v.contains('\\')
        || v.contains('<')
        || v.contains('(')
        || v.contains("eyj")
        || k.contains("redirect")
        || k.contains("debug")
        || k.contains("password")
        || k.contains("passwd")
        || k.contains("file")
        || k.contains("fn")
        || k.contains("template")
        || k.contains("
