// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Minimal HTML helpers
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
const CSS_BODY: &str = "body { font-family: sans-serif; margin: 2rem; }";
const CSS_H1: &str = "h1 { border-bottom: 1px solid #ccc; padding-bottom: .5rem; }";
const CSS_UL: &str = "ul { list-style: none; padding: 0; }";
const CSS_LI: &str = "li { margin: .4rem 0; }";
const CSS_A: &str = "a { text-decoration: none; color: #0070c0; }";
const CSS_A_HOVER: &str = "a:hover { text-decoration: underline; }";
const CSS_TABLE: &str = "table { border-collapse: collapse; width: 100%; font-size: .9rem; }";
const CSS_TH_TD: &str = "th, td { border: 1px solid #ddd; padding: .4rem .6rem; text-align: left; }";
const CSS_TH: &str = "th { background: #f0f0f0; }";
const CSS_TR_NTH: &str = "tr:nth-child(even) { background: #fafafa; }";
const CSS_CLASS_BACK: &str = ".back { margin-bottom: 1rem; display: block; }";
const CSS_CLASS_ERROR: &str = ".error {{ color: #c00; }}";

pub fn gen_page(page_title: &str, body: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8"/>
  <meta name="viewport" content="width=device-width, initial-scale=1"/>
  <title>{page_title}</title>
  <style>
    {CSS_BODY}
    {CSS_H1}
    {CSS_UL}
    {CSS_LI}
    {CSS_A}
    {CSS_A_HOVER}
    {CSS_TABLE}
    {CSS_TH_TD}
    {CSS_TH}
    {CSS_TR_NTH}
    {CSS_CLASS_BACK}
    {CSS_CLASS_ERROR}
  </style>
</head>
<body>
  <h1>{page_title}</h1>
  {body}
</body>
</html>"#
    )
}
