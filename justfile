test:
    cargo odra test

build:
    cargo odra build

deploy:
    cargo run --bin deploy --features odra-livenet
