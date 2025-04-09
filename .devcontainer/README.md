# Dev container

Install devpod:
```
curl -L -o devpod "https://github.com/loft-sh/devpod/releases/latest/download/devpod-linux-amd64" && \
  install -c -m 0755 devpod .local/bin && \
  rm -f devpod
```

Start your favorite IDE:
```
devpod up . --ide zed
```

Note: the first time takes longer to build the container.
