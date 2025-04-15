# Use CentOS Stream to build.
FROM quay.io/centos/centos:stream9 AS builder

# Install build dependencies from CentOS repos.
RUN dnf -y --setopt=install_weak_deps=0 --enablerepo=crb install \
    cargo pkg-config perl-FindBin openssl-devel perl-lib perl-IPC-Cmd perl-File-Compare perl-File-Copy tpm2-tss-devel clang-devel protobuf-compiler \
    tar gzip

# Install build dependencies from Intel repo.
WORKDIR /root
RUN curl -O https://download.01.org/intel-sgx/sgx-linux/2.24/distro/centos-stream9/sgx_rpm_local_repo.tgz && \
    echo "91b11cee5d1f6b65a9a5fb31b8b8168da98af2af685abda34c5d709f80cd7704 sgx_rpm_local_repo.tgz" | sha256sum --check && \
    tar -xaf sgx_rpm_local_repo.tgz && \
    dnf -y install --nogpgcheck --repofrompath "sgx,file:///root/sgx_rpm_local_repo" libsgx-dcap-quote-verify-devel

# Install development tools.
RUN dnf -y --setopt=install_weak_deps=0 install \
    git rust-src clippy rustfmt openssh-server
