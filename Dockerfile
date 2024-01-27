# Using base dev image
FROM ssleo/base-dev-image:latest

# Install "run on save" tool
RUN apt install -y inotify-tools

# Time zone configuration
RUN ln -snf /usr/share/zoneinfo/$CONTAINER_TIMEZONE /etc/localtime && echo $CONTAINER_TIMEZONE > /etc/timezone

COPY . /app

RUN chown -R admin:admin /app

# Setting default user
USER admin

# Install Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# Setting default work directory
WORKDIR /app

# Expose server port
EXPOSE 9090