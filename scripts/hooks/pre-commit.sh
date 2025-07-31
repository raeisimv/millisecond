#!/bin/bash

#just build;
if ! just build ; then
#if [ $? -ne 0 ]; then
    echo "pre-commit: build failed"
    exit 1
else
    echo "pre-commit: build succeeded";
fi
