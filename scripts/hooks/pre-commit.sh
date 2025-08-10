#!/bin/bash

if ! just build ; then
    echo "pre-commit: build failed"
    exit 1
else
    echo "pre-commit: build succeeded";
fi
