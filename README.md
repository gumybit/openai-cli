# OpenAI CLI

A tool to easily run openai api with CLI. made with Rust.

## Install

```sh
brew tap gumybit/openai-cli
brew install openai-cli
openci-ali -h
```

## Usage

- set your API token to `OPEN_AI_API_TOKEN`

```
$ openai-cli -h

Usage: openai-cli [OPTIONS] <COMMAND>

Commands:
  completion  
  chat        
  help        Print this message or the help of the given subcommand(s)

Options:
  -d, --debug...  Turn debuggin information on
  -h, --help      Print help
  -V, --version   Print version
```

### Chat

- basic usage

```sh
openai-cli chat "Your message"
```

- full options

```
$ openai-cli chat -h

Usage: openai-cli chat [OPTIONS] <MESSAGE> [ROLE]

Arguments:
  <MESSAGE>  message passing to ChatGPT
  [ROLE]     role passing to ChatGPT [default: user]

Options:
  -m, --model <MODEL>              [default: gpt-3.5-turbo]
      --max-tokens <MAX_TOKENS>    
      --temperature <TEMPERATURE>  0 to 2. What sampling temperature to use
      --top-p <TOP_P>              0 to 1. the model considers the results of the tokens with top_p probability mass
      --n <N>                      How many completions to generate for each prompt
      --user <USER>                A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse [default: user]
  -h, --help                       Print help
```

### Completion

- basic usage

```sh
openai-cli completion "Your prompt"
```

- full options

```
$ openai-cli completion -h

Usage: openai-cli completion [OPTIONS] <PROMPT>

Arguments:
  <PROMPT>  prompt passing to ChatGPT

Options:
  -m, --model <MODEL>              [default: text-davinci-003]
  -s, --suffix <SUFFIX>            The suffix that comes after a completion of inserted text
      --max-tokens <MAX_TOKENS>    [default: 16]
      --temperature <TEMPERATURE>  0 to 2. What sampling temperature to use
      --top-p <TOP_P>              0 to 1. the model considers the results of the tokens with top_p probability mass
      --n <N>                      How many completions to generate for each prompt
  -h, --help                       Print help
```

## Welcome to PRs / Issues

Yes, I'm welcome to your PRs and Stars. :custard:

## License

MIT

## Author

gumybit
