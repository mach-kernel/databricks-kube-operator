FROM debian:bullseye-slim

RUN apt update
RUN apt install -y tini

ENTRYPOINT ["/usr/bin/tini", "--"]

WORKDIR /home/operator

ADD target/release/crd_gen .
ADD target/release/databricks_kube .

RUN chmod +x crd_gen
RUN chmod +x databricks_kube

ENV RUST_LOG databricks_kube
CMD ["/home/operator/databricks_kube"]
