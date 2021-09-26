FROM rustembedded/cross:x86_64-unknown-linux-musl-0.2.1
RUN curl -fsSL https://deb.nodesource.com/setup_lts.x | bash - \
    && curl -sL https://dl.yarnpkg.com/debian/pubkey.gpg | gpg --dearmor | tee /usr/share/keyrings/yarnkey.gpg >/dev/null \
    && echo "deb [signed-by=/usr/share/keyrings/yarnkey.gpg] https://dl.yarnpkg.com/debian stable main" | tee /etc/apt/sources.list.d/yarn.list \
    && apt-get update \
    && apt-get -y install yarn \
    && rm -rf /var/lib/apt/lists/*
