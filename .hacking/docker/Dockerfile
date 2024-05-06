FROM --platform=$BUILDPLATFORM golang:alpine AS build
ARG TARGETPLATFORM
ARG BUILDPLATFORM

# Set the working directory
WORKDIR /app

RUN echo "I am running on $BUILDPLATFORM, building for $TARGETPLATFORM" > /log

# Download binary
RUN wget https://github.com/quarylabs/quary/releases/download/0.0.102/quary-0.0.102-quary-linux-aarch64-gnu.zip

# Install unzip
RUN apk add --no-cache unzip
# Unzip binary
RUN unzip /app/quary-0.0.102-quary-linux-aarch64-gnu.zip

RUN ls -la

FROM --platform=$BUILDPLATFORM alpine

WORKDIR /app

COPY --from=build /app/quary /app/quary

# Set the binary as the entrypoint
ENTRYPOINT ["quary"]
