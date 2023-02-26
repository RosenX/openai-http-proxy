

export DATABASE_URL="mysql://root:1234@localhost:3306/feedbox_dev"

sea-orm-cli migrate refresh

# Generate entity files of database `bakeries_db` to `src/entities`
# sea-orm-cli generate entity -o src/entities

# sea-orm-cli migrate up