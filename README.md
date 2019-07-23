# jsl-validate

`jsl-validate` validates lines of JSON against a JSL schema, and outputs each
line's errors. Because `jsl-validate` sets a nonzero status code if any of the
inputs were invalid, you can use it in the `if` of a bash script.

## Usage

See `jsl-validate --help` for detailed help, but essentially you run it like so:

```text
$ cat schema.json
{
  "properties": {
    "foo": { "type": "string" },
    "bar": {
      "elements": { "type": "string" }
    }
  }
}

$ cat inputs.json
{ "foo": "xxx", "bar": ["a", "b", "c"] }
{ "foo": "xxx", "bar": ["a", 123, "c"] }
{ "foo": false, "bar": ["a", "b", "c"] }

$ jsl-validate schema.json inputs.json
[]
[{"instancePath":"/bar/1","schemaPath":"/properties/bar/elements/type"}]
[{"instancePath":"/foo","schemaPath":"/properties/foo/type"}]
```
