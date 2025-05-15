1. See the implementation of the mongoDB directly from the actix website under databases
2. Rethink the user + authentication storage to be Postgres
3. Mongo indexing implement and force data set data to be a path to a S3 storage location -> use this as extension
    - In general keep these principles in mind:
    1. Mongo documents store mostly meta data. The more the better. Details regarding who owns them and everything is stored within a RBAC style hierarchy
    2. The actual data if small - about 1MB keep it with the document within the mongoDB for retrieval
    3. If the data is > 1MiB then use a Kafka stream to move the source data into a S3 location. This should then be refernces in the MongoDB storage
    4. Kafka streams should handle data movement for anything. The API itself takes in a data response but the details regarding how this data is handled need to happen
    within the interface. Separation of conerns means that the API has all of the tools to move the data but only the UX determines how these tools are used.

