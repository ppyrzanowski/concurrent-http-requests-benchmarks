#!/bin/bash
PORT=8000
APP_ROOT="."
SRC_PATH="${APP_ROOT}"
ENV_PATH="${SRC_PATH}/env"
APP_NAME="server"
# Make sure /var/log/gunicorn exists!
ACCESS_LOGFILE="${APP_ROOT}/logs/${APP_NAME}_access.log"
ERROR_LOGFILE="${APP_ROOT}/logs/${APP_NAME}.log"

mkdir -p logs

# Gunicorn logs to stderr, while the wsgi-app logs to stdout.

exec ${ENV_PATH}/bin/gunicorn \
  --bind="127.0.0.1:${PORT}" \
  --access-logformat="%({X-Real-IP}i)s %(l)s %(u)s %(t)s \'%(r)s\' %(s)s %(b)s \'%(f)s\' \'%(a)s\'" \
  --access-logfile="$ACCESS_LOGFILE" \
  --error-logfile="$ERROR_LOGFILE" \
  --chdir="$APP_ROOT" \
  --worker-class=gevent \
  "${APP_NAME}:app"
  # --log-level="warning"

echo $!
