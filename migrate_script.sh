

export DATABASE_URL="mysql://root:1234qwer@localhost:3306/feed_inbox"

# sea-orm-cli migrate refresh

# Generate entity files of database `bakeries_db` to `src/entities`
# sea-orm-cli generate entity -o src/entities


sea-orm-cli migrate up