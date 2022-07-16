# gh-ranking

Show in-organization ranking of GitHub activities such as review count.

## Installation

```
gh extension install yukukotani/gh-ranking
```

## Usage

```
USAGE:
    gh-ranking [OPTIONS] <ORGANIZATION> <ACTION>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -q, --query <query>    Additional GitHub search query
    -t, --team <team>      Team ID

ARGS:
    <ORGANIZATION>    Organization ID
    <ACTION>          open-pr | review-pr
```

### Example: Ranking of review count in engineering team since 2022-06-01

```
gh ranking ubie-inc review-pr --team engineering --query 'created:>=2022-06-01'
```
