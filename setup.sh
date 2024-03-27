#!/bin/env sh

chmod 744 .githooks/pre-commit
chmod 744 .githooks/pre-commit.wrapper
cp .githooks/pre-commit.wrapper .git/hooks/pre-commit
chmod 744 .git/hooks/pre-commit
