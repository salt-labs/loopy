##################################################
# Notes for GitHub Actions
#       * Dockerfile instructions: https://git.io/JfGwP
#       * Environment variables: https://git.io/JfGw5
##################################################

# REMINDER: export DOCKER_BUILDKIT=1

#########################
# Build Args
#########################

ARG USERNAME="apt-mirror"

ARG VERSION="latest"
ARG VARIANT="bookworm-slim"

#########################
# Runner
#########################

FROM docker.io/debian:${VARIANT} AS runner

# hadolint ignore=DL3018,DL3008
RUN export DEBIAN_FRONTEND="noninteractive" \
	&& apt-get update \
	&& apt-get install -y \
	--no-install-recommends \
	apt-utils \
	&& apt-get upgrade -y \
	--no-install-recommends \
	&& apt-get install -y \
	bash \
	bc \
	ca-certificates \
	coreutils \
	curl \
	debmirror \
	git \
	gnupg \
	gpg \
	tar \
	wget \
	zip \
	&& update-ca-certificates \
	&& apt clean -y \
	&& rm -rf /var/lib/apt/lists/*

COPY "scripts" "/scripts"

ENTRYPOINT ["/scripts/entrypoint.sh"]
CMD ["debmirror"]
