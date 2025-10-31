#!/bin/bash

git rm -rf ./gh-pages/
trunk build --release --public-url "/polymesh-api-example-leptos" -d ./gh-pages/
cp gh-pages/index.html gh-pages/404.html
git add ./gh-pages/

