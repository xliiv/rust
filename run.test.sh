#!/usr/bin/env bash

set -ex

./x.py test --incremental --stage 1 --verbose rustdoc --test-args intra-link-self --keep-stage 1 | tee run.test.log
#./x.py test --incremental --stage 1 --verbose rustdoc --test-args intra-link-self | tee run.test.log

#echo HTML
#ls -la build/x86_64-unknown-linux-gnu/test/rustdoc/intra-link-self
#firefox build/x86_64-unknown-linux-gnu/test/rustdoc/intra-link-self/foo
