# mini-bank

Mini-bank is a project to showcase rust capability in building fast microservices. It's a small bank project where customers shall be able to to make deposits and withdrawal maintaining simplified General Ledgers. 
The project is not intended to follow any ISO standards and its basically for demo purpose

## Contributing

[![GitHub stars](Contributing)](https://github.com/rust-nairobi/mini-bank/stargazers/)
If you like what we do, consider starring, commenting, sharing and contributing!

## Tooling

+ [Actix](https://github.com/actix)
+ [GraphQL](https://github.com/graphql)
+ [Sea-orm](https://github.com/SeaQL/sea-orm)
+ [Postgres Database](https://github.com/postgres/postgres)

## Features
### Development features:
We shall have microservices 

1. User service

    This service will incorporate users functionalities including customers auth and auth, admins

2. Transactions service

    This will be built to interface core banking and any communicating channels (Web UI) to enable customers to carry out transactions deposit and withdrawals

3. Messaging and Callback service

    This will be a notification service and a callabck service as well.
    It will run background check to complete async processes and close pending and hanging transactions

 4. Web UI

 	This will serve as an internet banking web interface to help customers transact and view their account. The admins will be offered an interface to control and set variuos configurations within the platform 



