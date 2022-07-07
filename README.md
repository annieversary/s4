# super simple static site (s4)

this is a super simple static site generator with multi-language support, with minimal configuration

it uses [tera](https://tera.netlify.app/) as it's template engine, and strings are defined in `toml` files

## important files and folders

- `templates`: tera template files
- `templates/pages`: tera template files which will be rendered
- `static`: for css, js, images, and other. will be directly copied into `out`
- `langs`: toml files containing the translations
- `out`: the output directory
- `s4.toml`: config file

## pages

only the templates inside `templates/pages` will be rendered, so you can have a base file at `templates/base.html`, and it won't be rendered

## languages

languages are defined by creating `.toml` files in the `langs` directory. these will be fed directly into the templates

## variables

- `lang`: current language code
- `available_langs`: list of languages
- every string in the language toml files

## filters

- `link`: if `lang` is `en`, then `{{ "hello" | link}}` will be `/en/hello`

## config

configuration is defined in `s4.toml`. available keys are:

- `default_lang`: language that will be rendered at the root of the output folder. it will also be rendered inside it's directory

## example

there's an example in `example_site`. if you want to run it:

```bash
[s4] $ cargo b
[s4] $ cd example_site
[s4/example_site] $ ../target/debug/s4
# if you have python installed, you can use the following:
[s4/example_site] $ python3 -m "http.server" --directory out
```
