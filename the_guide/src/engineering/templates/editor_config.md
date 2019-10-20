# Editor Config

```
# EditorConfig is awesome: https://EditorConfig.org

# top-most EditorConfig file
root = true

# Unix-style newlines with a newline ending every file
[*]
end_of_line = lf
insert_final_newline = true
indent_style = space
indent_size = 2
trim_trailing_whitespace = true
insert_final_newline = true
max_line_length = 10


# Tab indentation (no size specified)
[Makefile]
indent_style = tab

[*.{js,jsx,ts,tsx,vue,json}]
indent_size = 2

# 4 space indentation
[{*.rs,*.toml,*.sane,*.json, *.sql}]
indent_size = 4
```
