FROM lang_envs

WORKDIR /judger
COPY src src
COPY rust-judger ./
ENV BASE_PATH=/judger

CMD ["/judger/rust-judger"]



