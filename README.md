# regex_generate.py

**Use regular expressions to generate text.**
This is the PyO3 Python binding of the Rust crate [regex_generate](https://github.com/CryptArchy/regex_generate).

## Installation
```bash
pip install regex-generate-py
```

## Usage

```python
>>> from regex_generate_py import generate
>>> 
>>> pattern = r"(My|Your) name is an(ji|na|t)"
>>> for i in range(10):
...     print(generate(pattern))
...
Your name is anna
My name is ant
Your name is anji
Your name is anji
My name is ant
Your name is anna
My name is ant
My name is anji
Your name is anji
My name is ant
```
