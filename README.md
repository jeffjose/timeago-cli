<p>
  <a href="#">
    <img alt="A dead-simple CLI to convert RFC2332 to short humanize strings" src="https://raw.github.com/jeffjose/timeago-cli/main/banner.png">
  </a>
</p>

# timeago-cli

A dead-simple CLI to convert RFC2332 to short humanize strings. Useful for prompts

## Usage

```bash
$ timeago 'Wed, 21 Oct 2020 00:07:08 -0700'
```

Use it with `git log` to get how far back the last commit was

```bash
$ timeago "$(git log -1 --format=%cD)"
12h
```

You can get longer strings with `--long`

```bash
$ timeago "$(git log -1 --format=%cD)" --long
12 hours
```

Or precise timeago strings with `--precise`

```bash
$ timeago "$(git log -1 --format=%cD)" --precise
12 hours 30 minutes and 19 seconds
```

Customize the separator (default ` `) with `--seperator`

```bash
$ timeago "$(git log -1 --format=%cD)" --precise --separator '|'
12h|31m|43s
```

## Install

```bash
cargo install timeago-cli
```