# !/bin/sh
#export DATABASE_URL="mysql://rocket:password@localhost/app"
export DATABASE_URL=postgres://postgres:$PASSWORD@$HOST/app
export ROCKET_DATABASES="{ app = { \"$DATABASE_URL\" } }"
#export ROCKET_DATABASES="{ app = { \"$DATABASE_URL\" } }"