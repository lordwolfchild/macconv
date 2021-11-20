# macconv

This is a small tool to parse and convert MAC addresses from one format to another. IT takes input of any of the known MAC notations and then outputs it in colon based notation by default, or to dashed and Cisco Notation through command line parameters.

It was meant as a small excercise in Rust programming. But the result seems to be rather useful, so I'll put it up here for you to use.

## Structure

Standard cargo based project.

## Install

Via Cargo inside the project directory:

``` bash
$ cargo install
```

## Usage

```
$ macconv --help
macconv 1.0.0

normalizes and converts mac addresses.

USAGE:
    macconv [FLAGS] <mac>

FLAGS:
    -C, --caps       output mac using capital letters
    -w, --cisco      output mac in Cisco notation
    -c, --colon      output mac in colon notation
    -d, --dashed     output mac in dashed notation
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <mac>    A MAC Address to work on
```
## Examples

```
$ macconv ff:23:ca:fd:ab:aa -w
ff23.cafd.abaa
```

```
$ macconv ff23.cafd.abaa -d -C
FF-23-CA-FD-AB-AA
```

```
macconv FF-23-CA-FD-ab-aA -c
ff:23:ca:fd:ab:aa
```

## Change log

V1.0: initial version.

## License

The MIT License (MIT). Please see [License File](LICENSE.md) for more information.

