use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use wasm_bindgen::prelude::*;

const COMMANDS: &[&str] = &[
    "ML",
    "ML_command",
    "ML_export",
    "ML_file",
    "ML_file_debug",
    "ML_file_no_debug",
    "ML_val",
    "SML_export",
    "SML_file",
    "SML_file_debug",
    "SML_file_no_debug",
    "SML_import",
    "abbreviation",
    "alias",
    "attribute_setup",
    "axiomatization",
    "bibtex_file",
    "bnf",
    "bundle",
    "chapter",
    "class",
    "class_deps",
    "codatatype",
    "code_datatype",
    "code_deps",
    "code_identifier",
    "code_monad",
    "code_pred",
    "code_printing",
    "code_reflect",
    "code_reserved",
    "code_thms",
    "coinductive",
    "coinductive_set",
    "compile_generated_files",
    "consts",
    "context",
    "copy_bnf",
    "corollary",
    "datatype",
    "datatype_compat",
    "declaration",
    "declare",
    "default_sort",
    "definition",
    "end",
    "experiment",
    "export_code",
    "export_generated_files",
    "external_file",
    "extract",
    "extract_type",
    "find_consts",
    "find_theorems",
    "find_unused_assms",
    "free_constructors",
    "full_prf",
    "fun",
    "fun_cases",
    "function",
    "functor",
    "generate_file",
    "global_interpretation",
    "help",
    "hide_class",
    "hide_const",
    "hide_fact",
    "hide_type",
    "inductive",
    "inductive_cases",
    "inductive_set",
    "inductive_simps",
    "instance",
    "instantiation",
    "interpretation",
    "judgment",
    "lemma",
    "lemmas",
    "lift_bnf",
    "lift_definition",
    "lifting_forget",
    "lifting_update",
    "local_setup",
    "locale",
    "locale_deps",
    "method_setup",
    "named_theorems",
    "nitpick_params",
    "no_notation",
    "no_syntax",
    "no_translations",
    "no_type_notation",
    "nonterminal",
    "notation",
    "notepad",
    "nunchaku_params",
    "old_rep_datatype",
    "oracle",
    "overloading",
    "paragraph",
    "parse_ast_translation",
    "parse_translation",
    "partial_function",
    "prf",
    "primcorec",
    "primcorecursive",
    "primrec",
    "print_ML_antiquotations",
    "print_abbrevs",
    "print_antiquotations",
    "print_ast_translation",
    "print_attributes",
    "print_bnfs",
    "print_bundles",
    "print_case_translations",
    "print_cases",
    "print_claset",
    "print_classes",
    "print_codeproc",
    "print_codesetup",
    "print_coercions",
    "print_commands",
    "print_context",
    "print_definitions",
    "print_defn_rules",
    "print_facts",
    "print_induct_rules",
    "print_inductives",
    "print_interps",
    "print_locale",
    "print_locales",
    "print_methods",
    "print_options",
    "print_orders",
    "print_quot_maps",
    "print_quotconsts",
    "print_quotients",
    "print_quotientsQ3",
    "print_quotmapsQ3",
    "print_record",
    "print_rules",
    "print_simpset",
    "print_state",
    "print_statement",
    "print_syntax",
    "print_term_bindings",
    "print_theorems",
    "print_theory",
    "print_trans_rules",
    "print_translation",
    "prop",
    "proposition",
    "quickcheck_generator",
    "quickcheck_params",
    "quotient_definition",
    "quotient_type",
    "realizability",
    "realizers",
    "record",
    "schematic_goal",
    "section",
    "setup",
    "setup_lifting",
    "simproc_setup",
    "sledgehammer_params",
    "smt_status",
    "specification",
    "subclass",
    "sublocale",
    "subparagraph",
    "subsection",
    "subsubsection",
    "syntax",
    "syntax_declaration",
    "term",
    "termination",
    "text",
    "text_raw",
    "theorem",
    "theory",
    "thm",
    "thm_deps",
    "thm_oracles",
    "thy_deps",
    "translations",
    "txt",
    "typ",
    "type_alias",
    "type_notation",
    "type_synonym",
    "typed_print_translation",
    "typedecl",
    "typedef",
    "unbundle",
    "unused_thms",
    "value",
    "values",
    "welcome",
];

fn is_outer_cmd(key: &str) -> bool {
    COMMANDS.binary_search(&key).is_ok()
}

const ISACOMMAND: &str = "isacommand";
const ISAOPEN: &str = "isacartoucheopen";
const ISACLOSE: &str = "isacartoucheclose";
const ISANEWLINE: &str = "isanewline";
const ISAPARENR: &str = "isacharparenright";
const ISAPRIME: &str = "isacharprime";
const ISAKERN: &str = "kern0pt";

const BEGIN: &str = "SNIP";

fn begin_snippet(name: &str) -> String {
    vec!["\\", BEGIN, "{", name, "}{"].join("")
}

fn end_snippet() -> String {
    String::from("}")
}

fn cartouche(name: &str, line: usize, n: usize) -> String {
    format!("{{\\Cartouche{{{}}}{{{}}}{{{}}}}}", name, line, n)
}

fn escape_underscores(s: &str) -> String {
    s.replace("_", "-")
}

// Isabelle names can look like prefix{\isacharunderscore}{\kern0pt}suffix
// Or like {\isasymphi}prop1
fn take_isaname(chars: &[char]) -> Option<String> {
    let mut name: Vec<char> = vec![];
    let mut i = 0;

    let ctrl: Vec<char> = "\\isactrl".chars().collect();

    while i < chars.len() {
        let c = chars[i];
        if c.is_alphanumeric() {
            name.push(c);
            i += 1;
        } else if chars[i..].starts_with(&ctrl) {
            i = skip_char(chars, i, ' ');
        } else if c == '{' {
            let k = skip_char(&chars, i, '}');
            let cmd = String::from_iter(&chars[i..k]);

            if cmd == "{\\isacharunderscore}" {
                name.push('_');
                // Skip the "{\kern0pt}"
                i = skip_char(&chars, k, '}');
            } else if cmd == "{\\isacharprime}" && !name.is_empty() {
                name.push('\'');
                // Skip the "{\kern0pt}"
                i = skip_char(&chars, k, '}');
            } else if cmd.starts_with("{\\isasym") {
                let sym = &cmd[8..cmd.len() - 1];
                name.extend(sym.chars());
                i = k;
            } else if cmd.starts_with("{\\isadigit{") {
                let digit = &cmd[11..cmd.len() - 1];
                name.extend(digit.chars());
                // Skip the extra '}' here
                i = k + 1;
            } else {
                break;
            }
        } else {
            break;
        }
    }
    let ret = name.iter().collect::<String>();
    if ret.is_empty() {
        None
    } else {
        Some(ret)
    }
}

fn read_until_cmd(
    chars: &[char],
    mut i: usize,
    goal: &str,
    pred: Option<fn(&str) -> bool>,
) -> Option<usize> {
    while i < chars.len() {
        let c = chars[i];
        let mut k = 1;

        if c == '\\' {
            if i + 1 < chars.len() && chars[i + 1] == ' ' && goal == " " {
                return Some(i);
            }

            let mut cmd = vec![];
            while i + k < chars.len() {
                let d = chars[i + k];

                if d == '\\' || d == '{' || d == '}' || d == '%' || d.is_ascii_whitespace() {
                    break;
                }
                cmd.push(d);

                k += 1;
            }
            let cmd_name = cmd.iter().collect::<String>();
            if cmd_name == goal {
                cmd.clear();
                k += 1;

                match pred {
                    Some(p) => match take_isaname(&chars[i + k..]) {
                        Some(inner) if p(&inner) => return Some(i),
                        _ => {}
                    },
                    None => return Some(i),
                }
            }
        }
        i += k;
    }

    None
}

fn chunk_theory(chars: &[char]) -> Vec<Vec<char>> {
    let mut res = vec![];
    let mut i = 1;

    while let Some(k) = read_until_cmd(&chars, i, ISACOMMAND, Some(is_outer_cmd)) {
        let chunk = Vec::from(&chars[i - 1..k - 1]);
        res.push(chunk);
        i = k + 1;
    }

    res
}

fn skip_char(chars: &[char], mut i: usize, g: char) -> usize {
    while i < chars.len() && chars[i] != g {
        i += 1;
    }
    i + 1
}

fn cmd_chunk(chars: &[char]) -> String {
    if let Some(mut i) = read_until_cmd(chars, 0, ISACOMMAND, Some(is_outer_cmd)) {
        i = skip_char(chars, i, '{');
        take_isaname(&chars[i..]).unwrap_or(String::new())
    } else {
        String::new()
    }
}

fn skip_whitespace(chars: &[char], mut i: usize) -> usize {
    let nl: Vec<char> = format!("\\{}", ISANEWLINE).chars().collect();

    while i < chars.len() {
        if chars[i] == '\n' {
            i += 1;
        } else if chars[i] == '\\' && chars[i + 1] == ' ' {
            i += 2;
        } else if chars[i..].starts_with(&nl) {
            i += nl.len();
        } else {
            break;
        }
    }

    i
}

fn name_chunk(chars: &[char]) -> String {
    let cmd = cmd_chunk(chars);

    // Go for a name directly after the command, e.g. lemma NAME
    if let Some(mut i) = read_until_cmd(chars, 0, " ", None) {
        i += 2;
        i = skip_whitespace(&chars, i);
        if let Some(name) = take_isaname(&chars[i..]) {
            return name;
        }
    }

    // Go for a name after parentheses, e.g. function (sequential) NAME
    if let Some(mut i) = read_until_cmd(chars, 0, ISAPARENR, None)
        .and_then(|i| read_until_cmd(chars, i, ISAKERN, None))
    {
        let entered_cartouche = read_until_cmd(chars, 0, ISAOPEN, None)
            .map(|k| k < i)
            .unwrap_or(false);

        if !entered_cartouche {
            i = skip_char(&chars, i, '}');
            i = skip_whitespace(&chars, i);

            if let Some(name) = take_isaname(&chars[i..]) {
                return name;
            }
        }
    }

    // Go for a name after a type variable, e.g. datatype 'a NAME = ...
    if cmd == "datatype" || cmd == "codatatype" || cmd == "type_synonym" {
        if let Some(mut i) = read_until_cmd(chars, 0, ISAPRIME, None)
            .and_then(|i| read_until_cmd(chars, i, ISAKERN, None))
        {
            i = skip_char(&chars, i, '}');
            // There will be a space after the type variable name but no space in front of it
            i = skip_char(&chars, i, ' ');
            i = skip_whitespace(&chars, i);

            if let Some(name) = take_isaname(&chars[i..]) {
                return name;
            }
        }
    }

    // Go for a name inside cartouches, e.g. abbrevation "NAME ..."
    if cmd == "abbreviation" || cmd == "definition" {
        if let Some(mut i) = read_until_cmd(chars, 0, ISAOPEN, None) {
            i = skip_char(&chars, i, '}');
            i = skip_whitespace(&chars, i);

            if let Some(name) = take_isaname(&chars[i..]) {
                return name;
            }
        }
    }

    // Use the hash of the snippet as name

    let mut hasher = DefaultHasher::new();
    chars.hash(&mut hasher);
    let hash = hasher.finish();
    format!("{:x}", hash)
}

fn line_chunk(chars: &[char]) -> Vec<String> {
    let mut lines = vec![];
    let mut current = vec![];

    for &c in chars {
        current.push(c);
        if c == '\n' {
            lines.push(String::from_iter(&current));
            current.clear();
        }
    }

    if !current.is_empty() {
        let s = String::from_iter(current);
        lines.push(s);
    }

    let mut filtered_lines = vec![];

    for line in lines {
        if line.trim_end() == "%" {
            continue;
        } else if line.trim_end() == "\\" {
            filtered_lines.push(String::from("\\ %\n"));
        } else {
            filtered_lines.push(line);
        }
    }

    let nl = format!("\\{}", ISANEWLINE);

    let mut res = vec![];
    let mut chunk = vec![];

    // Ignore markup commands
    let markup = String::from("\\isamarkup");
    let markupfalse = String::from("\\isamarkupfalse");
    // Text commands can sit in the middle of proofs so we just make sure to get a separate line for these
    let begin_text = String::from("\\begin{isamarkuptext}");
    let end_text = String::from("\\end{isamarkuptext}");
    for line in filtered_lines {
        if line.contains(&nl) || line.contains(&end_text) {
            chunk.push(line);
            res.push(chunk.concat());
            chunk.clear();
        } else if line.contains(&begin_text) {
            if !chunk.is_empty() {
                res.push(chunk.concat());
                chunk.clear();
            }
            chunk.push(line);
        } else if line.contains(&markup) && !line.contains(&markupfalse) {
            break;
        } else {
            chunk.push(line);
        }
    }

    if !chunk.is_empty() {
        res.push(chunk.concat());
    }

    // Strip lines without any visible content
    while let Some(last) = res.pop() {
        if has_visible_content(&last) {
            // Remove trailing isanewlines from the last line
            let fixed = last
                .trim_end_matches("\\isanewline")
                .trim_end_matches("\\isanewline\n");
            if !fixed.is_empty() {
                res.push(fixed.to_string());
            }
            break;
        }
    }

    // Strip blank lines
    res.into_iter().map(|s| s.trim_end().to_string()).collect()
}

fn has_visible_content(s: &str) -> bool {
    if s.contains("\\isachar")
        || s.contains("\\isasym")
        || s.contains("\\isacommand")
        || s.contains("\\isacartouche")
    {
        return true;
    }

    let mut last = ' '; // Trick so start of line behaves like whitespace.

    for c in s.chars() {
        if last.is_whitespace() && c.is_alphabetic() {
            return true;
        }
        last = c;
    }

    false
}

fn lift_cartouches(start: &str, full: &str) -> Vec<String> {
    let start_chars: Vec<char> = start.chars().collect();
    let full_chars: Vec<char> = full.chars().collect();
    let mut cartouches = vec![];
    let mut i = 0;

    while let Some(j) = read_until_cmd(&start_chars, i, ISAOPEN, None) {
        i = skip_char(&start_chars, j, '}');

        if let Some(k) = read_until_cmd(&full_chars, i, ISACLOSE, None) {
            let cart = String::from_iter(&full_chars[i..k - 1]);
            cartouches.push(cart);

            i = skip_char(&full_chars, k, '}') + 1;
        }
    }

    cartouches
}

#[wasm_bindgen]
pub fn extract_snippets(s: String, theory: String) -> String {
    let chars: Vec<char> = s.chars().collect();
    let chunks = chunk_theory(&chars);

    let mut used_names: HashMap<String, usize> = HashMap::default();
    let mut snippets: Vec<String> = vec![];

    for chunk in chunks.into_iter().skip(1) {
        let cmd = &cmd_chunk(&chunk);
        let name = name_chunk(&chunk);
        let raw_id = format!("{}:{}", cmd, name);

        let suffix = used_names.entry(raw_id.clone()).or_insert(0);
        let id = if *suffix > 0 {
            escape_underscores(&format!("{}-{}", &raw_id, suffix))
        } else {
            escape_underscores(&raw_id)
        };
        *suffix += 1;

        let prefix;
        if !theory.is_empty() {
            prefix = format!("{}:{}", escape_underscores(&theory), id);
        } else {
            prefix = id;
        }

        let lines = line_chunk(&chunk);
        for (i, line) in lines.iter().enumerate() {
            let line_name = format!("{}-{}", prefix, i);
            let mut lifted_line = line.clone();

            let cartouches = lift_cartouches(line, &lines[i..].concat());
            for (k, cart) in cartouches.iter().enumerate() {
                let cart_name = format!("{}-{}", line_name, k);
                snippets.push(begin_snippet(&cart_name));
                snippets.push(cart.clone());
                snippets.push(end_snippet());

                let pat = format!("{{\\{}}}{}{{\\{}}}", ISAOPEN, cart, ISACLOSE);
                lifted_line = lifted_line.replace(&pat, &cartouche(&prefix, i, k));
            }

            snippets.push(begin_snippet(&line_name));
            snippets.push(lifted_line);
            snippets.push(end_snippet());
        }
    }

    // Newline at the end
    snippets.push(String::new());
    snippets.join("%\n")
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    Ok(())
}
