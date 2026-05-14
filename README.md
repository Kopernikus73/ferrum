# Ferrum

A simple chess bot for your command line.

## About Ferrum

Ferrum is a research project where I am trying to find out how to find "the best" chess moves fast without using LLMs or Coding AI.\
This is not trying to come close to any great chess bot like stockfish and not trying to be "the fastest" or "the best".\
It's just aiming to find out what methods there are to optimize a chess bot.

## Installation
### Manual installation on Linux (recommended) 
#### Requirements
- git
- rust (cargo)

To manually install ferrum just download this git repository and build the binary using cargo

Download the repository\
`$ git clone git@github.com:Kopernikus73/ferrum.git -o ferrum`

Change directory to ferrum\
`$ cd ferrum`

Build the binary using Cargo\
`$ cargo build --release`

The binary is located in `/target/release` as `ferrum`

## License

As software should always be available for free for everyone, this project uses the GNU AGPLv3 License.

#### License specifications
| Permissions                                        | Conditions                                                      | Limitations                                 |
|----------------------------------------------------|-----------------------------------------------------------------|---------------------------------------------|
| <span style="color:green;">✓</span> Commercial use | <span style="color:cyan;">ⓘ</span> Disclose source              | <span style="color:red;">✕</span> Liability |
| <span style="color:green;">✓</span> Distribution   | <span style="color:cyan;">ⓘ</span> License and copyright notice | <span style="color:red;">✕</span> Warranty  |
| <span style="color:green;">✓</span> Modification   | <span style="color:cyan;">ⓘ</span> Network use is distribution  |                                             |
| <span style="color:green;">✓</span> Patent use     | <span style="color:cyan;">ⓘ</span> Same license                 |                                             |
| <span style="color:green;">✓</span> Private use    | <span style="color:cyan;">ⓘ</span> State changes                |                                             |

Further information: [gnu.org](https://www.gnu.org), [choosealicense.com](https://choosealicense.com/licenses/)