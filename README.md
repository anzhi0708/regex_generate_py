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
>>> pattern = r"(My|Your|His|Her) name is an(ji|na|t|imal) with number \-?[0-9]{2,6}"
>>> for i in range(10):
...     print(generate(pattern))
...
```
```text
My name is animal with number 194
His name is animal with number 24843
Her name is anna with number -79
My name is animal with number 60585
Your name is animal with number 71877
Your name is ant with number 41
Her name is anna with number -16320
My name is animal with number 353970
His name is anna with number 42
Your name is ant with number -040```
