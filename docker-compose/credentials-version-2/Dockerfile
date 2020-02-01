ARG GREMLIN_SERVER

FROM tinkerpop/gremlin-server:${GREMLIN_SERVER}

COPY gremlin-server-credentials-v2.yaml /opt/gremlin-server/conf/
COPY server.jks /opt/gremlin-server/conf/
