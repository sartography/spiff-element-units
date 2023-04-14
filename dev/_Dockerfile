FROM python:alpine AS base

RUN \
    --mount=type=cache,target=/var/cache/apk \
    apk add -U make

RUN python -m pip install --upgrade pip

WORKDIR /app

COPY dev/requirements.txt .

RUN pip install --no-cache-dir -r requirements.txt