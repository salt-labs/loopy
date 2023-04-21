
FROM harbor.poc.dcse.mil.au/docker.io/debian:11-slim

RUN echo "Installing packages..." \
 && apt update \
 && apt install -y debmirror gpg \
 && apt clean -y \
 && rm -rf /var/cache/apt/archives /var/lib/apt/lists

COPY entrypoint.sh /

ENTRYPOINT ["/entrypoint.sh"]
CMD ["debmirror"]

