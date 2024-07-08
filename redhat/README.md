To update the `rpms.lock.yaml` file, proceed as follows.

You need to be in a registered RHEL system.
```
subscription-manager register
This system is already registered.
```

Enable repos in `redhat.repo`:
```
subscription-manager repos \
--enable rhel-9-for-x86_64-appstream-rpms \
--enable rhel-9-for-x86_64-appstream-source-rpms \
--enable codeready-builder-for-rhel-9-x86_64-rpms \
--enable codeready-builder-for-rhel-9-x86_64-source-rpms
```

The source repos are necessary to ship them in the source image.

Install the command:
```
pip install --user https://github.com/konflux-ci/rpm-lockfile-prototype/archive/refs/heads/main.zip
```

Update the lock file:
```
rpm-lockfile-prototype -f kbs/docker/rhel-ubi/Dockerfile redhat/rpms.in.yaml
```
