use std::{
    collections::HashMap,
    env::args,
    ffi::OsStr,
    fs::{read_dir, read_to_string, File},
    io::Write,
    path::{Path, PathBuf},
};

use color_eyre::{
    eyre::{eyre, WrapErr},
    Result,
};
use tera::{from_value, Context, Tera};
use toml::Value;

fn main() -> Result<()> {
    color_eyre::install()?;

    let arg = args().nth(1);
    if arg.as_deref() == Some("-v") || arg.as_deref() == Some("--version") {
        println!(
            "s4 {} {} {}",
            env!("VERGEN_BUILD_SEMVER"),
            env!("VERGEN_GIT_SHA_SHORT"),
            env!("VERGEN_BUILD_DATE")
        );
        return Ok(());
    }

    // load
    let config = load_config()?;
    let langs = load_langs()?;
    println!("found {} languages", langs.len());
    let mut tera = Tera::new("templates/**/*.html")?;
    println!(
        "found {} pages",
        tera.get_template_names()
            .filter(|n| n.starts_with("pages/"))
            .count()
    );

    // remove old stuff
    if Path::new("./out").exists() {
        std::fs::remove_dir_all("./out").wrap_err("out directory does not exist")?;
    }

    let available_langs = langs.iter().map(|s| s.0.as_str()).collect::<Vec<_>>();

    if !available_langs.contains(&config.default_lang.as_str()) {
        return Err(eyre!(
            "default_lang '{}' was not found in 'langs'",
            &config.default_lang
        ));
    }

    render_lang(
        &mut tera,
        &config.default_lang,
        &langs.iter().find(|s| s.0 == config.default_lang).unwrap().1,
        &available_langs,
        true,
    )?;
    println!("rendered default language ({})", &config.default_lang);
    for (code, value) in &langs {
        render_lang(&mut tera, &code, value, &available_langs, false)?;
        println!("rendered {}", code);
    }

    if Path::new("./static").exists() {
        std::process::Command::new("/bin/sh")
            .args(["-c", "cp -r static/* out"])
            .output()
            .wrap_err("failed to copy static directory")?;
        println!("static files copied");
    }
    println!("done");
    Ok(())
}

#[derive(serde_derive::Deserialize)]
struct Config {
    default_lang: String,
}

fn load_config() -> Result<Config> {
    let s = read_to_string("s4.toml")?;
    Ok(toml::from_str(&s)?)
}

pub type Lang = (String, Value);
fn load_langs() -> Result<Vec<Lang>> {
    let mut res = vec![];
    for e in read_dir("./langs")? {
        let e = e?;
        let p = e.path();

        if e.file_type()?.is_file() && p.extension() == Some(OsStr::new("toml")) {
            let lang_code = p.file_stem().unwrap().to_string_lossy().to_string();
            let s = read_to_string(&p)?;
            let val = s.parse::<Value>()?;

            res.push((lang_code, val));
        }
    }

    Ok(res)
}

fn render_lang(
    tera: &mut Tera,
    code: &str,
    value: &Value,
    available_langs: &[&str],
    render_at_root: bool,
) -> Result<()> {
    let mut context = Context::from_serialize(value)?;
    context.insert("lang", code);
    context.insert("available_langs", available_langs);

    let code_clone = code.to_string();
    tera.register_filter(
        "link",
        move |path: &tera::Value, _args: &HashMap<String, tera::Value>| {
            let path = from_value::<String>(path.clone())?;
            Ok(tera::Value::String(format!(
                "/{code_clone}/{}",
                path.strip_prefix("/").unwrap_or(&path),
            )))
        },
    );

    for template in tera
        .get_template_names()
        .filter(|n| n.starts_with("pages/"))
    {
        let content = tera
            .render(template, &context)
            .wrap_err_with(|| format!("error rendering language: {code}"))?;

        let name = template
            .strip_prefix("pages/")
            .expect("the template name to start with 'pages/'")
            .strip_suffix(".html")
            .unwrap_or(template);

        let mut out_path = PathBuf::from("./out");
        if !render_at_root {
            out_path.push(code);
        }

        if !name.ends_with("index") {
            out_path.push(name);
        }

        std::fs::create_dir_all(&out_path)?;

        out_path.push("index.html");

        let mut file = File::create(out_path)?;
        file.write_all(content.as_bytes())?;
    }

    Ok(())
}
