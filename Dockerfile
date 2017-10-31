# docker run -ti -v $PWD:/opt rust
FROM ubuntu:16.04
RUN apt-get update -qq && apt-get install -y curl
RUN useradd user --create-home
RUN curl https://sh.rustup.rs > rustup 
RUN chmod +x rustup
RUN chown -R user:user /opt
USER user
RUN ./rustup -y --default-toolchain nightly-x86_64
WORKDIR /opt
RUN /bin/bash -c "source ~/.profile && rustup target add aarch64-unknown-linux-gnu"
ENTRYPOINT [ "/bin/bash", "-c", "source ~/.profile && cargo build" ]
