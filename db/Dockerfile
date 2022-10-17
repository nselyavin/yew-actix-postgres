FROM postgres:14.5
ENV POSTGRES_PASSWORD=1324

COPY up.sql /docker-entrypoint-initdb.d/init.sql
# COPY init.db.sh /tmp/init.db.sh

# RUN ["/tmp/init.db.sh"]