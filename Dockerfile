FROM lang_envs

WORKDIR /judger
COPY src src
COPY target/release/rust-judger ./
ENV BASE_PATH=./

CMD ["/judger/rust-judger"]



