#!/bin/bash

# sea-orm-cli generate entity --database-url postgres://autarch:autarch@localhost/crumb \
#     --database-schema crumb \
#     --output-dir ./src/crumb \
#     --with-serde

sea-orm-cli -v generate entity --database-url postgres://autarch:autarch@localhost/crumb \
    --database-schema musicbrainz \
    --output-dir ./src/musicbrainz \
    --with-serde
