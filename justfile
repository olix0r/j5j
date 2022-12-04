
lint: action-lint clippy

##
## Rust
##

profile := 'debug'
target := 'x86_64-unknown-linux-gnu'

fetch:
    just-cargo fetch --locked

clippy: fetch
    just-cargo clippy --frozen

build: fetch
    just-cargo profile='{{ profile }}' target={{ target }} build --frozen

package dir : build
    @mkdir -p '{{ dir / profile }}'
    tar czf "{{ dir / profile }}/j5j-$(just crate-version)-{{ target }}.tar.gz" \
        -C '{{ "target" / target / profile }}' \
        j5j

crate-version:
    @just-cargo crate-version j5j

action-lint:
    @just-dev lint-actions
