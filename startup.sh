#export DATABASE_URL="mysql://rocket:password@localhost/app"
export DATABASE_URL="postgres://postgres:password@localhost/app"
export ROCKET_DATABASES="{ app = { url = \"$DATABASE_URL\" } }"