#!/bin/bash

git rm -rf ./gh-pages/
trunk build --release --public-url "/polymesh-api-example-leptos" -d ./gh-pages/
git add ./gh-pages/

