<p>
  <a href="#">
    <img alt="A dead-simple CLI to convert RFC2332 to short humanize strings" src="https://raw.github.com/jeffjose/timeago-cli/main/banner.png">
  </a>
</p>

# timeago-cli

A dead-simple CLI to convert RFC2332 to short humanize strings. Useful for prompts

## Usage

```
$ timeago 'Wed, 21 Oct 2020 00:07:08 -0700'
```

Use it with `git log` to get how far back the last commit was

```
$ timeago "$(git log -1 --format=%cD)"
2m ago
```

## Install