# j5j

Reads [JSON5] from one or more files and prints it as plain old JSON.

Based on `json5-to-json by @callum-oakley.

## Examples

```console
:; j5j .devcontainer/devcontainer.json
{"extensions":["DavidAnson.vscode-markdownlint","kokakiwi.vscode-just","NathanRidley.autotrim","redhat.vscode-yaml","rust-lang.rust-analyzer","samverschueren.final-newline","tamasfe.even-better-toml"],"image":"ghcr.io/linkerd/dev:v36","mounts":["source=/var/run/docker.sock,target=/var/run/docker-host.sock,type=bind"],"name":"j5j","overrideCommand":false,"remoteUser":"code","runArgs":["--init","--memory=12g","--memory-swap=12g","--net=host"]}
```

```console
:; j5j --pretty .devcontainer/devcontainer.json
{
  "extensions": [
    "DavidAnson.vscode-markdownlint",
    "kokakiwi.vscode-just",
    "NathanRidley.autotrim",
    "redhat.vscode-yaml",
    "rust-lang.rust-analyzer",
    "samverschueren.final-newline",
    "tamasfe.even-better-toml"
  ],
  "image": "ghcr.io/linkerd/dev:v36",
  "mounts": [
    "source=/var/run/docker.sock,target=/var/run/docker-host.sock,type=bind"
  ],
  "name": "j5j",
  "overrideCommand": false,
  "remoteUser": "code",
  "runArgs": [
    "--init",
    "--memory=12g",
    "--memory-swap=12g",
    "--net=host"
  ]
}
```

[JSON5]: https://json5.org/
[json5-rs]: https://github.com/callum-oakley/json5-rs
[Serde JSON]: https://github.com/serde-rs/json
