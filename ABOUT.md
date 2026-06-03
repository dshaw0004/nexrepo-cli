### Recommended Crates

| Purpose              | Crate                                                                                                          |
| -------------------- | -------------------------------------------------------------------------------------------------------------- |
| CLI framework        | [`clap`](https://github.com/clap-rs/clap) (derive feature)                                                     |
| Interactive prompts  | [`inquire`](https://github.com/mikaelmello/inquire)                                                            |
| Colored output       | [`colored`](https://github.com/mackwic/colored)                                                                |
| Config storage       | [`serde`](https://github.com/serde-rs/serde)                                                                   |
| Shell out to git     | [`std::process::Command`](https://doc.rust-lang.org/std/process/struct.Command.html) (stdlib, no crate needed) |
| Cross-platform paths | [`dirs`](https://github.com/dirs-dev/dirs-rs) for `~/.nexrepo/`                                                |
| Progress Bars        | [`indicatif`](https://docs.rs/indicatif)                                                                       |

---

### Project Structure

```
nexrepo-cli/
├── src/
│   ├── main.rs
│   ├── commands/
│   │   ├── init.rs
│   │   ├── push.rs
│   │   ├── pull.rs
│   │   ├── sync.rs
│   │   └── status.rs
│   ├── config.rs      # read/write ~/.nexrepo/config.toml
│   ├── git.rs         # git shell-out helpers
│   └── providers/
│       ├── github.rs
│       └── gitlab.rs
└── Cargo.toml
```
