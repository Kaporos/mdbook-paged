# mdbook-paged

## Instalation
    git clone https://github.com/Kaporos/mdbook-paged
    cd mdbook-paged
    cargo install --path .

## Configuration

Add this to your book.toml to run this tool on build :) 

```toml
[output.paged]
```

Your PDF output will be placed in book/pdf/output.pdf

## Running

By default, the pdf will be built only if `GEN_PDF` env variable is set. 

    GEN_PDF=1 mdbook build

if you want mdbook-paged to run either is env is not set, just add

```toml

[output.paged]
always=true
```

and pdf will be built automatically at each mdbook build.

### Injecting into HTML

If you want to inject your pdf to the html website print feature, just enable the config like this:

```toml
[output.paged]
inject-html = true
```

And then your pdf will be placed at book/html/output.pdf and will be accessible using html print feature !  

