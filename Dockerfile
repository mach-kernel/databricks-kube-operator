FROM alpine:latest

RUN apk add --no-cache tini
ENTRYPOINT ["/sbin/tini", "--"]

WORKDIR /home/operator

ADD target/release/crd_gen .
ADD target/release/databricks_kube .

RUN chmod +x crd_gen
RUN chmod +x databricks_kube

ENV RUST_LOG databricks_kube
CMD ["databricks_kube"]
