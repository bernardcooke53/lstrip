# uncomment

## Installation

```bash
cargo install --git https://github.com/bernardcooke53/uncomment.git@main uncomment
```

## Usage

```bash
# Copy your favourite programmer's commented commands out of
# some file

uncomment "$(cat<<EOF
# ls .
# echo "important message"
EOF
)" | bash
```

You can also use `uncomment` with other comment styles, such a "//":

```bash
cat <<EOF | uncomment "//" | exec
// ls .
// echo "important message"
EOF
```

## Known issues

Shell redirection currently doesn't seem to work, i.e. you can't do

```bash
echo "// foo" | uncomment "//"
```

which I want to fix at some point, so things can properly be chained.

## Contribution

PRs welcome. I may publish this on crates.io at some point but for the moment
it's not there.
