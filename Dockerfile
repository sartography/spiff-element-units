FROM python:alpine AS base

RUN \
    --mount=type=cache,target=/var/cache/apk \
    apk add -U make

WORKDIR /app