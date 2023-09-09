#!/bin/bash -e

RUST_PARALLEL="./target/debug/rust-parallel"
VERSION=$($RUST_PARALLEL -V | cut -f2 -d' ')

echo "## Manual for rust-parallel $VERSION"

echo '
1. [Command line](#command-line)
1. [Commands from arguments](#commands-from-arguments)
1. [Commands from stdin](#commands-from-stdin)
1. [Parallelism](#parallelism)
1. [Debug logging](#debug-logging)
1. [Timeout](#timeout)
1. [Progress bar](#progress-bar)
1. [Specifying command and initial arguments on command line](#specifying-command-and-initial-arguments-on-command-line)
1. [Reading multiple inputs](#reading-multiple-inputs)
1. [Regular Expression](#regular-expression)
1. [Bash Function](#bash-function)
'

echo '## Command line'

echo '```
$ rust-parallel --help'
$RUST_PARALLEL --help
echo '```'

echo '## Commands from arguments.

The `:::` separator can be used to run the [Cartesian Product](https://en.wikipedia.org/wiki/Cartesian_product) of command line arguments.  This is similar to the `:::` behavior in GNU Parallel.
'

echo '```
$ rust-parallel echo ::: A B ::: C D ::: E F G'
$RUST_PARALLEL echo ::: A B ::: C D ::: E F G

echo '
$ rust-parallel echo hello ::: larry curly moe'
$RUST_PARALLEL echo hello ::: larry curly moe

echo '
# run gzip -k on all *.html files in current directory
$ rust-parallel gzip -k ::: *.html
```'

echo '## Commands from stdin.

Run complete commands from stdin.

'
echo '```
$ cat >./test <<EOL
echo hi
echo there
echo how
echo are
echo you
EOL'
cat >./test<<EOL
echo hi
echo there
echo how
echo are
echo you
EOL

echo '
$ cat test | rust-parallel'
cat test | $RUST_PARALLEL

rm -f test

echo '```'

echo '
## Parallelism

By default the number of parallel jobs to run simulatenously is the number of cpus detected at run time.

This can be override with the `-j`/`--jobs` option.

With `-j5` all echo commands below run in parallel.

With `-j1` all jobs run sequentially.
'

echo '```
$ rust-parallel -j5 echo ::: hi there how are you'
$RUST_PARALLEL -j5 echo ::: hi there how are you

echo '
$ rust-parallel -j1 echo ::: hi there how are you'
$RUST_PARALLEL -j1 echo ::: hi there how are you

echo '```'

echo '## Debug logging.

Set environment variable `RUST_LOG=debug` to see debug output.

This logs structured information about command line arguments and commands being run.

Recommend enabling debug logging for all examples to understand what is happening in more detail.
'

echo '```
$ RUST_LOG=debug rust-parallel echo ::: hi there how are you | grep command_line_args | head -1'
RUST_LOG=debug $RUST_PARALLEL echo ::: hi there how are you | grep command_line_args | head -1 | ansi-stripper

echo '
$ RUST_LOG=debug rust-parallel echo ::: hi there how are you | grep 'command_line_args:1''
RUST_LOG=debug $RUST_PARALLEL echo ::: hi there how are you | grep 'command_line_args:1' | ansi-stripper
echo '```'

echo '## Timeout.

The `-t` option can be used to specify a command timeout in seconds:
'

echo '```'

echo '$ rust-parallel -t 0.5 sleep ::: 0 3 5'
$RUST_PARALLEL -t 0.5 sleep ::: 0 3 5 | ansi-stripper

echo '```'

echo '## Progress bar.

The `-p` option can be used to enable a graphical progress bar.

This is best used for commands which are running for at least a few seconds, and which do not produce output to stdout or stderr.

In the below command `-d all` is used to discard all output from commands run:'

echo '```
$ rust-parallel -d all -p sleep ::: 1 2 3'
echo '⠤ [00:00:01] Commands Done/Total:  1/3  █████████░░░░░░░░░░░░░░░░░░ ETA 00:00:02'
echo '```'

echo '## Specifying command and initial arguments on command line:

Here `md5 -s` will be prepended to each input line to form a command like `md5 -s aal`
'

echo '```
$ head -100 /usr/share/dict/words | rust-parallel md5 -s | head -10'
head -100 /usr/share/dict/words | $RUST_PARALLEL md5 -s | head -10
echo '```
'

echo '## Reading multiple inputs.

By default `rust-parallel` reads input from stdin only.  The `-i` option can be used 1 or more times to override this behavior.  `-i -` means read from stdin, `-i ./test` means read from the file `./test`:
'

echo '```'
echo '$ cat >./test <<EOL
foo
bar
baz
EOL'
cat >./test <<EOL
foo
bar
baz
EOL

echo '
$ head -5 /usr/share/dict/words | rust-parallel -i - -i ./test echo'
head -5 /usr/share/dict/words | $RUST_PARALLEL -i - -i ./test echo

rm -f test

echo '```'

echo '
## Regular Expression

Regular expressions can be specified by the `-r` or `--regex` command line argument.

[Named or numbered capture groups](https://docs.rs/regex/latest/regex/#grouping-and-flags) are expanded with data values from the current input before the command is executed.

In these examples using command line arguments `{url}` and `{filename}` are named capture groups.  `{0}` is a numbered capture group.
'

echo '```'
echo -e '$ rust-parallel -r \x27(?P<url>.*),(?P<filename>.*)\x27 echo got url={url} filename={filename} ::: URL1,filename1  URL2,filename2'
$RUST_PARALLEL -r '(?P<url>.*),(?P<filename>.*)' echo got url={url} filename={filename} ::: URL1,filename1  URL2,filename2

echo
echo -e '$ rust-parallel -r \x27(?P<url>.*) (?P<filename>.*)\x27 echo got url={url} filename={filename} full input={0} ::: URL1 URL2 ::: filename1 filename2'
$RUST_PARALLEL -r '(?P<url>.*) (?P<filename>.*)' echo got url={url} filename={filename} full input={0} ::: URL1 URL2 ::: filename1 filename2

echo '```'

echo 'In the next example input file arguments `{0}` `{1}` `{2}` `{3}` are numbered capture groups, and the input is a csv file:'

echo '```'
echo '$ cat >./test <<EOL
foo,bar,baz
foo2,bar2,baz2
foo3,bar3,baz3
EOL'
cat >./test <<EOL
foo,bar,baz
foo2,bar2,baz2
foo3,bar3,baz3
EOL

echo
echo -e '$ cat test | rust-parallel -r \x27(.*),(.*),(.*)\x27 echo got arg1={1} arg2={2} arg3={3} full input={0}'
cat test | $RUST_PARALLEL -r '(.*),(.*),(.*)' echo got arg1={1} arg2={2} arg3={3} full input={0}

echo '```'

rm -f test

echo '## Bash Function

Use `-s` shell mode to invoke an arbitrary bash function.

Similar to normal commands bash functions can be called using stdin, input files, or from command line arguments.'

echo '### Function Setup

Define a bash fuction `logargs` that logs all arguments and make visible with `export -f`:
'

echo '```'

echo '$ logargs() {
  echo "logargs got $@"
}'
logargs() {
  echo "logargs got $@"
}

echo '
$ export -f logargs'
export -f logargs

echo '```'

echo '### Demo of command line arguments:
'

echo '```
$ rust-parallel -s logargs ::: A B C ::: D E F'
$RUST_PARALLEL -s logargs ::: A B C ::: D E F

echo '```'

echo '### Demo of function and command line arguments from stdin:'

echo '```
$ cat >./test <<EOL
logargs hello alice
logargs hello bob
logargs hello charlie
EOL'
cat >./test <<EOL
logargs hello alice
logargs hello bob
logargs hello charlie
EOL

echo '
$ cat test | rust-parallel -s'

cat test | $RUST_PARALLEL -s
rm -f test

echo '```
'

echo '### Demo of function and initial arguments on command line, additional arguments from stdin:'

echo '```
$ cat >./test <<EOL
alice
bob
charlie
EOL'
cat >./test <<EOL
alice
bob
charlie
EOL

echo '
$ cat test | rust-parallel -s logargs hello'

cat test | $RUST_PARALLEL -s logargs hello
rm -f test

echo '```
'