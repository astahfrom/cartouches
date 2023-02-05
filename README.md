Cartouches
==========

Parses Isabelle LaTeX output into manageable snippets.

Setup
-----

I assume you have build a PDF document of your LaTeX files with `isabelle`.
This produces a number of `.tex`-files, typically in `./output/document/`.

This tool converts those LaTeX files into a single file containing more manageable snippets.

See the `index.html` file for more information.

Command Line Usage
------------------

To use the project on the command line, simply run it on either a single file:

```bash
cargo run INPUT.tex OUTPUT.tex
```

or on a directory:

```bash
cargo run INPUTS/ OUTPUT.tex
```

Web Integration
---------------

To compile the tool to JavaScript, run:

```bash
wasm-pack build --target web
```

See `index.html` for the integration with HTML.
